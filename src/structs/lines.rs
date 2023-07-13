use crate::structs::*;
use std::mem::transmute;

/// A square whose rows or columns have been collapsed into one by folding with BITWISE OR
#[derive(Clone, Copy)]
pub struct SubSection([Cell; 3]);

impl Square {
    /// Folds the rows and columns of a SQUARE using `OR`.
    /// Rotates if required and returns `[rows, cols]`.
    /// # Safety
    /// This function should only be called on `Squares`
    pub fn fold_into_subsections(self, rotate: bool) -> [Cell; 3] {
        let cells = self.to_cells();
        let rows = [
            cells[0] | cells[1] | cells[2],
            cells[3] | cells[4] | cells[5],
            cells[6] | cells[7] | cells[8],
        ];
        let cols = [
            cells[0] | cells[3] | cells[6],
            cells[1] | cells[4] | cells[7],
            cells[2] | cells[5] | cells[8],
        ];
        if !rotate {
            rows
        } else {
            cols
        }
    }
}

impl From<SubSection> for [Cell; 3] {
    fn from(value: SubSection) -> Self {
        value.0
    }
}

impl SubSection {
    /// Takes the three subsections of a square and a number `0 <= N <= 3`.
    /// Within each bit stores a `1` if we have seen `N` `1`s at that bit position,
    /// else stores a `0`.
    /// ```rust
    /// let bits = SubSection([
    ///     0b0101_0010,
    ///     0b0011_1010,
    ///     0b0010_0011,
    /// ]);         
    /// assert!(bits.contain_count<0>() = 0b1000_0100);
    /// assert!(bits.contain_count<1>() = 0b0101_1001);
    /// assert!(bits.contain_count<2>() = 0b0011_0000);
    /// assert!(bits.contain_count<3>() = 0b0000_0010);
    /// ```
    /// # Safety
    /// `N` should technically be represented as an `Idx<4>`,
    /// but this is not allowed as a const generic. Panics if `N > 3`.
    pub fn contain_count<const N: u8>(&self) -> Cell {
        let [x, y, z] = self.0;

        let bit_and = x & y & z;
        let bit_or = x | y | z;
        let bit_xor = x ^ y ^ z;

        match N {
            0 => !bit_or,
            1 => bit_xor & !bit_and,
            2 => bit_or & !bit_or,
            3 => bit_and,
            4.. => unreachable!(),
        }
    }
}

/// Indexes into the 6 areas of a `Grid`
/// # REPRESENTATION
/// `0   1   2`
/// `↓   ↓   ↓`
/// `S │ S │ S <- 3`
/// `──┼───┼──`
/// `S │ S │ S <- 4`
/// `──┼───┼──`
/// `S │ S │ S <- 5`
pub type AreaIdx = Idx<6>;

/// Represents a collection of 3 squares.
/// ```rust
/// pub struct Area {
///     pub values: [SubSection; 3], // values of the three `Squares`
///     pub masks: [[Mask; 3]; 3],   // masks to create `Filter`s
///     pub index: AreaIdx,          // its own `AreaIdx`, used for outputting `Filter`s
/// }
/// ```
/// # Comment
/// The squares are thought to be horizontal.
/// If the are represents a vertical alignment of squares
/// they are rotated counterclockwise about the bottom.
/// # Safety
/// Beware that `grid.get_area()` utilises unitialised forms of `Area`.
/// Any modification to the fields of `Area` should be immediatelly matched
/// within said method.
/// # Representation
/// (possibly rotated) squares used in construction:
/// `Cell Cell Cell | Cell Cell Cell | Cell Cell Cell`
/// `Cell Cell Cell | Cell Cell Cell | Cell Cell Cell`
/// `Cell Cell Cell | Cell Cell Cell | Cell Cell Cell`
/// ## `Area.values`
/// combine the candidates of each column into one (using `OR` on columns)
/// `Cell | Cell | Cell`
/// `Cell | Cell | Cell`
/// `Cell | Cell | Cell`
/// ## `Area.masks`
/// Bitmasks for each of the 9 `Subsection`s. 0..=2 for the first square and so on.
/// ## `Area.index`
/// Keeps track of its `AreaIdx`.
/// Used when getting indices to return the final `Filter`s.
pub struct Area {
    pub values: [SubSection; 3],
    pub masks: [[Mask; 3]; 3],
    pub index: AreaIdx,
}

// border chars: ─ │ ┌ ┐ ┘ └ ┼

impl Grid {
    /// Obtains the `Area` from within the `Grid`, rotating if necessary
    pub fn get_area(&self, n: AreaIdx) -> Area {
        let rotate = u8::from(n) < 3;
        // get the cells in the 3 squares
        let square_idxs: [SectionIdx; 3] = unsafe {
            std::mem::transmute(match u8::from(n) {
                0 => [0_u8, 3, 6],
                1 => [1, 4, 7],
                2 => [2, 5, 8],
                3 => [0, 1, 2],
                4 => [3, 4, 5],
                5 => [6, 7, 8],
                _ => unreachable!(),
            })
        };

        let squares: [Square; 3] = square_idxs.map(|i| {
            let cell_idxs = Grid::square_indices(i);
            Square::new(self.get_cells(cell_idxs))
        });

        let values = squares.map(|s| SubSection(s.fold_into_subsections(rotate)));

        Area {
            values,
            masks: Default::default(),
            index: n,
        }
    }
}

impl Area {
    /// Check pairs of columns in data for matching candidate signatures per column
    /// E.g.: "First and second squares both have "`101`" as their candidate signature for the
    /// number 2 (in terms of their columns)" => First `Cell` at the 2nd bit (0 indexed) is a 1.
    /// If they differ then the bit would be zero.
    /// Ordering: `0 <-> 1, 0 <-> 2, 1 <-> 2`
    /// For each pair: `!XOR` columns then `AND` rows
    /// `Cell | Cell     Cell`
    /// `Cell | Cell --> Cell --> Cell`
    /// `Cell | Cell`    Cell`
    pub fn get_matches(&self) -> [Cell; 3] {
        fn collapsed_square_equality(a: SubSection, b: SubSection) -> Cell {
            let (a, b) = (a.0, b.0); // use inner values
            !(a[0] ^ b[0]) & !(a[1] ^ b[1]) & !(a[2] ^ b[2])
        }

        let values = self.values;

        [
            collapsed_square_equality(values[0], values[1]),
            collapsed_square_equality(values[0], values[2]),
            collapsed_square_equality(values[1], values[2]),
        ]
    }

    /// Known numbers within the squares represented as candidates within a `Cell`
    pub fn get_known(&self) -> [Cell; 3] {
        self.values.map(|square| Cell::combine_known(&square.0))
    }

    /// Uses `Subsection::contains_count()` on each `Square`.
    pub fn get_count<const N: u8>(&self) -> [Cell; 3] {
        self.values.map(|s| s.contain_count::<1>())
    }

    /// Updates `values` using `masks`
    /// # TODO
    /// possibly flatten `self.values` and `self.masks` to begin with
    /// # Comment
    /// It seems that these transmutes are free looking at the asm output.
    pub fn update_values(&mut self) {
        let values: &mut [Cell; 27] = unsafe { transmute(&mut self.values) };
        // Making this a reference avoids copying the data.
        let masks: &[Mask; 27] = unsafe { transmute(&self.masks) };

        for (cell, mask) in values.iter_mut().zip(masks) {
            *cell &= *mask
        }
    }

    /// Returns the `Filter`s given by `Area.masks`
    /// # TODO
    /// keep cache of original masks and only add a filter if it has something to filter
    /// and return a `Vec<Filter>`
    pub fn get_filters(&self) -> [Filter; 27] {
        // the 27 grid indexes pointing to the cells used to make `Area.values`
        let grid_idxs_flattened: [GridIdx; 27] = {
            let squares_idxs = match u8::from(self.index) {
                0 => [0, 3, 6],
                1 => [1, 4, 7],
                2 => [2, 5, 8],
                3 => [0, 1, 2],
                4 => [3, 4, 5],
                5 => [6, 7, 8],
                _ => unreachable!(),
            };

            let grid_idxs = squares_idxs.map(|square_idx| {
                Grid::square_indices(unsafe { SectionIdx::new_unchecked(square_idx) })
            });

            unsafe { transmute(grid_idxs) }
        };

        // let mut filters: [MaybeUninit<Filter>; 9] = MaybeUninit::uninit_array();
        let masks: [Mask; 9] = unsafe { transmute(self.masks) };

        macro_rules! filter_builder {
            ($grid_idxs_idx:expr, $mask_idx:expr) => {
                Filter {
                    mask: masks[$mask_idx],
                    idx: grid_idxs_flattened[$grid_idxs_idx],
                }
            };
        }

        if u8::from(self.index) > 2 {
            [
                filter_builder!(0, 0),
                filter_builder!(1, 0),
                filter_builder!(2, 0),
                filter_builder!(3, 1),
                filter_builder!(4, 1),
                filter_builder!(5, 1),
                filter_builder!(6, 2),
                filter_builder!(7, 2),
                filter_builder!(8, 2),
                filter_builder!(9, 3),
                filter_builder!(10, 3),
                filter_builder!(11, 3),
                filter_builder!(12, 4),
                filter_builder!(13, 4),
                filter_builder!(14, 4),
                filter_builder!(15, 5),
                filter_builder!(16, 5),
                filter_builder!(17, 5),
                filter_builder!(18, 6),
                filter_builder!(19, 6),
                filter_builder!(20, 6),
                filter_builder!(21, 7),
                filter_builder!(22, 7),
                filter_builder!(23, 7),
                filter_builder!(24, 8),
                filter_builder!(25, 8),
                filter_builder!(26, 8),
            ]
        } else {
            [
                filter_builder!(0, 2),
                filter_builder!(1, 1),
                filter_builder!(2, 0),
                filter_builder!(3, 2),
                filter_builder!(4, 1),
                filter_builder!(5, 0),
                filter_builder!(6, 2),
                filter_builder!(7, 1),
                filter_builder!(8, 0),
                filter_builder!(9, 5),
                filter_builder!(10, 4),
                filter_builder!(11, 3),
                filter_builder!(12, 5),
                filter_builder!(13, 4),
                filter_builder!(14, 3),
                filter_builder!(15, 5),
                filter_builder!(16, 4),
                filter_builder!(17, 3),
                filter_builder!(18, 8),
                filter_builder!(19, 7),
                filter_builder!(20, 6),
                filter_builder!(21, 8),
                filter_builder!(22, 7),
                filter_builder!(23, 6),
                filter_builder!(24, 8),
                filter_builder!(25, 7),
                filter_builder!(26, 6),
            ]
        }
    }
}
