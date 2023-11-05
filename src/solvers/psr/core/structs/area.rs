use super::{
    cell::{CellMask, ALL_CANDIDATES},
    filter::Filter,
    grid::Grid,
    idx::{AreaIdx, GridIdx, SectionIdx},
    section::{SectionInfo, SectionKind::Box},
};

pub struct FoldedArea {
    folds: [[u16; 3]; 3],
    masks: [[u16; 3]; 3],
    known: [u16; 3], // contain the known bit
    idx: AreaIdx,
}

const BOX_SECTION_IDXS: [[SectionIdx; 3]; 6] = unsafe {
    std::mem::transmute::<[[usize; 3]; 6], _>([
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
    ])
};

// Copy over to `grid.rs` when done
impl Grid {
    /// Constructs the [`FoldedArea`] at the location specified by the given index.
    pub fn get_folded_area(&self, area_idx: AreaIdx) -> FoldedArea {
        let boxes = BOX_SECTION_IDXS[usize::from(area_idx)] // indexes
            .map(|i| SectionInfo::new(Box, i)) // infos
            .map(|info| self.get_section(info)); // sections

        // this implementation keeps the "known bit" when accumulating
        let known = boxes.clone().map(|b| {
            b.cells
                .iter()
                .filter_map(|cell| cell.is_known().then_some(cell.to_u16()))
                .fold(0, |a, b| a | b)
        });

        let fold_hor = |b: [u16; 9]| [b[0] | b[1] | b[2], b[3] | b[4] | b[5], b[6] | b[7] | b[8]];
        let fold_ver = |b: [u16; 9]| [b[0] | b[3] | b[6], b[1] | b[4] | b[7], b[2] | b[5] | b[8]];

        let fold_func = if usize::from(area_idx) <= 2 {
            fold_ver
        } else {
            fold_hor
        };

        let folds = boxes.map(|b| unsafe { fold_func(std::mem::transmute(b.cells)) });

        FoldedArea {
            folds,
            masks: [[0; 3]; 3],
            known,
            idx: area_idx,
        }
    }
}

/// Core
impl FoldedArea {
    /// Turns inner masks into [Filters](Filter).
    /// Some filters may be redundant with respect to the board's state.
    pub fn get_filters(&self) -> [Filter; 27] {
        let mut grid_idxs = unsafe {
            std::mem::transmute::<_, [GridIdx; 27]>(
                BOX_SECTION_IDXS[usize::from(self.idx)]
                    .map(|i| SectionInfo::new(Box, i)) // infos
                    .map(|si| si.grid_idxs()), // grid idxs
            )
        };

        // if the area is vertical "transpose the boxes" via mem swaps
        if usize::from(self.idx) <= 2 {
            grid_idxs.swap(1, 3);
            grid_idxs.swap(2, 6);
            grid_idxs.swap(5, 7);
            grid_idxs.swap(10, 12);
            grid_idxs.swap(11, 15);
            grid_idxs.swap(14, 16);
            grid_idxs.swap(19, 21);
            grid_idxs.swap(20, 24);
            grid_idxs.swap(23, 25);
        }

        let masks = unsafe { std::mem::transmute::<_, [u16; 9]>(self.masks) };

        use std::mem::MaybeUninit;
        let mut filters: [MaybeUninit<Filter>; 27] = unsafe { MaybeUninit::uninit().assume_init() };
        for write_idx in 0..27 {
            let filter = Filter {
                idx: grid_idxs[write_idx],
                mask: unsafe { CellMask::new_unchecked(masks[write_idx / 3]) },
            };
            filters[write_idx] = MaybeUninit::new(filter);
        }

        unsafe { std::mem::transmute(filters) }
    }

    /// Applies the current masks to itself.
    /// Used after masks have been updated to continue computations with `Self`.
    pub fn apply_masks(&mut self) {
        for (folded_candidates, mask) in self
            .folds
            .iter_mut()
            .flatten()
            .zip(self.masks.iter().flatten())
        {
            *folded_candidates &= !mask;
        }
    }

    /// Uses the [`Grid`] to try to filter candidates further. Interally applies a more efficient
    /// implementation than chaining `FoldedArea::get_filters`, `Grid::apply_filters`, and
    /// `Grid::get_folded_area`. This method does not update the [`Grid`].
    ///
    /// Returns `true` if any filtering occured.
    pub fn update_known(&mut self, grid: &Grid) -> bool {
        todo!()
    }
}

/// Filters
impl FoldedArea {
    pub fn count<const N: u8>(&self) -> [u16; 3] {
        [0, 1, 2].map(|i| {
            let [x, y, z] = self.folds[i];

            let bit_and = x & y & z;
            let bit_xor = x ^ y ^ z;
            let bit_or = x | y | z;

            // flip candidate bits only; used to get the correct candidate masks
            let not_bit_and = bit_and ^ ALL_CANDIDATES;
            let not_bit_xor = bit_xor ^ ALL_CANDIDATES;
            let not_bit_or = bit_or ^ ALL_CANDIDATES;
            match N {
                0 => not_bit_or,
                1 => bit_xor & not_bit_and,
                2 => bit_or & not_bit_xor,
                3 => bit_and,
                _ => unreachable!(),
            }
        })
    }

    /// Checks for candidate projections in single lines.
    /// Returns `true` if it found any single candidates.
    pub fn single_lines(&mut self) -> bool {
        let mut updated = false;

        let mut count_1_unknown = self.count::<1>();
        for (a, b) in count_1_unknown.iter_mut().zip(self.known) {
            *a &= !b;
        }

        #[allow(clippy::needless_range_loop)] // the idx is decoupled to `count_1`
        for idx in 0..=2 {
            let c = count_1_unknown[idx];
            if c == 0 {
                continue;
            }

            // split each high bit in `c` into its own `u16`; collect into vec
            let candidates = (1..=9)
                .filter_map(|i| {
                    let candidate = 1 << i;
                    (c & candidate != 0).then_some(candidate)
                })
                .collect::<Vec<u16>>();

            for candidate in candidates {
                let row_idx = (0..=2)
                    .position(|i| self.folds[idx][i] & candidate != 0)
                    .unwrap();

                let compliment_idxs = [[1, 2], [0, 2], [0, 1]][idx];
                for comp_idx in compliment_idxs {
                    if self.folds[comp_idx][row_idx] & candidate != 0 {
                        self.masks[comp_idx][row_idx] |= candidate;
                        updated = true;
                    }
                }
            }
        }
        updated
    }

    /// Checks for candidate projections in double lines. Updates masks accordingly.
    pub fn double_lines(&mut self) -> bool {
        let mut updated = false;

        let count_2 = self.count::<2>();

        // the column at the current index is one that will (possibly) have candidates removed
        // i.e. we check for matching double lines in the remaining two columns
        for idx in 0..=2 {
            let compliment_idxs = [[1, 2], [0, 2], [0, 1]][idx];
            let [c2_a, c2_b] = compliment_idxs.map(|i| count_2[i]);
            let shared_bits = c2_a & c2_b;

            let shared_candidates = (1..=9)
                .filter_map(|i| {
                    let candidate = 1 << i;
                    (shared_bits & candidate != 0).then_some(candidate)
                })
                .collect::<Vec<u16>>();

            for candidate in shared_candidates {
                // > for each shared candidate we need the rows to also match up.
                // > we check for the row that does not contain the candidate
                let find_row_idx = |col_idx: usize| {
                    (0..=2)
                        .position(|row_idx| self.folds[col_idx][row_idx] & candidate == 0)
                        .unwrap()
                };
                let [row_a, row_b] = compliment_idxs.map(find_row_idx);

                // if the rows dont match or there is nothing to update, continue
                if row_a != row_b || self.folds[idx][row_a] & candidate == 0 {
                    continue;
                }

                self.masks[idx][row_a] |= candidate;
                updated = true;
            }
        }
        updated
    }
}
