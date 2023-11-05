use super::{
    area::FoldedArea,
    cell::{CandidateError, Cell, CellMask},
    filter::Filter,
    idx::{AreaIdx, GridIdx},
    num::Num,
    section::{Section, SectionInfo, SectionKind},
};

use crate::board::Board;

mod fmt; // Grid formatting
mod io; // Grid <-> Board parsing

/// The sudoku board for PSR solvers.
///
/// #### Nomenclature
/// A [`Cell`]'s "neighbours" are all other [`Cells`](Cell) in the grid that share a [`Section`]
/// with the given [`Cell`].
///
/// #### Grid Correctness
/// At any given time in a PSR Solver's computation, [`Cells`](Cell) within the Grid may contain
/// candidates that are "not allowed" due to neighboring known cells. This is a valid representation.
/// In these cases said [`Cells`](Cell) might have not been "updated" upon finding a value for a cell.
/// Care must be taken to promptly update [`Cells`](Cell) every time candidates are filtered or
/// values are found.
#[derive(PartialEq, Eq)]
pub struct Grid([Cell; 81]);

/// Grid-related errors
#[derive(Debug)]
pub enum GridError {
    /// Error in parsing a [`String`] into a [`Grid`].
    FromStringError,
}

// Manual implementation as we cannot derive Default with an inner array of size > 32.
impl Default for Grid {
    fn default() -> Self {
        Self([Cell::default(); 81])
    }
}

impl Grid {
    // ========================================================
    // Updating
    // ========================================================

    /// Creates filters for the neighbours of a cell when its value is found.
    /// The [`Num`] and [`GridIdx`] passed select the [`Cell`] and its value.
    ///
    /// # Note
    /// Technically there are only 20 [`Cells`](super::cell::Cell) to update when a known
    /// [`Cell`] is found due the overlap of [`Sections`](super::section::Section).
    /// However the overhead of removing these duplicates in the current implementation is greater
    /// than keeping them and filtering four [`Cells`](super::cell::Cell) twice.
    pub fn known_filters(n: Num, grid_idx: GridIdx) -> [Filter; 24] {
        use std::mem::MaybeUninit;

        let section_idxs = grid_idx.section_idxs();
        use SectionKind::*;
        let section_infos = [
            SectionInfo::new(Row, section_idxs[0]),
            SectionInfo::new(Column, section_idxs[1]),
            SectionInfo::new(Box, section_idxs[2]),
        ];

        let sections_grididxs = section_infos.map(|si| si.grid_idxs());
        let inner_idxs = grid_idx.inner_idxs();

        let mask = CellMask::from_known(n);
        let mut write_idx = 0;
        let mut filters: [MaybeUninit<Filter>; 24] = unsafe { MaybeUninit::uninit().assume_init() };

        for (section_grididxs, inner_idx) in sections_grididxs.into_iter().zip(inner_idxs) {
            for (i, grid_idx) in section_grididxs.into_iter().enumerate() {
                // skip the known cell
                if i == usize::from(inner_idx) {
                    continue;
                }

                let filter = Filter::new(mask, grid_idx);
                filters[write_idx] = MaybeUninit::new(filter);
                write_idx += 1;
            }
        }

        unsafe { std::mem::transmute(filters) }
    }

    /// Remove candidates at the specified location.
    pub fn apply_filter(&mut self, filter: Filter) {
        let cell = self.get_cell_mut(filter.idx);
        cell.remove_candidates(filter.mask);
    }

    /// Remove candidates at the specified locations in the [Grid].
    /// Applies [Grid::apply_filter](Grid::apply_filter) to each element in the given slice.
    pub fn apply_filters(&mut self, filters: &[Filter]) {
        for filter in filters {
            self.apply_filter(*filter);
        }
    }

    // ========================================================
    // Type Production
    // ========================================================

    /// Get a copy of the [`Cell`] the given index.
    #[inline(always)]
    pub fn get_cell(&self, idx: GridIdx) -> Cell {
        self.0[idx]
    }

    /// Get a mutable reference to the [`Cell`] at the given index.
    pub fn get_cell_mut(&mut self, idx: GridIdx) -> &mut Cell {
        &mut self.0[idx]
    }

    /// Returns (a copy of) the specified [`Section`].
    pub fn get_section(&self, section_info: SectionInfo) -> Section {
        use std::mem::MaybeUninit;
        let grid_idxs = section_info.grid_idxs();
        let mut uninit_cells: [MaybeUninit<Cell>; 9] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (cell, grid_idx) in uninit_cells.iter_mut().zip(grid_idxs) {
            *cell = MaybeUninit::new(self.get_cell(grid_idx));
        }

        let cells: [Cell; 9] = unsafe { std::mem::transmute(uninit_cells) };

        Section::new(section_info, cells)
    }

    /// Returns (copies of) the [`Sections`](Section) of the [`Cell`] at the given [`GridIdx`].
    pub fn get_sections(&self, grid_idx: GridIdx) -> [Section; 3] {
        let section_idxs = grid_idx.section_idxs();

        // a functional approach here is actually messier; so we stick to declarative
        use SectionKind::*;
        let row_info = SectionInfo::new(Row, section_idxs[0]);
        let col_info = SectionInfo::new(Column, section_idxs[1]);
        let box_info = SectionInfo::new(Box, section_idxs[2]);

        [row_info, col_info, box_info].map(|sec_info| self.get_section(sec_info))
    }

    /// Constructs the [`FoldedArea`] at the location specified by the given index.
    pub fn get_folded_area(&self, area_idx: AreaIdx) -> FoldedArea {
        use SectionKind::Box;
        let boxes = area_idx
            .to_box_section_idxs() // indexes
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

        FoldedArea::new(folds, known, area_idx)
    }

    // ========================================================
    // Solving
    // ========================================================

    /// Checks if a [`Cell`] contains a unique candidate within its three [`Section`]s.
    ///
    /// # Finder Method
    /// The function is a wrapper for a finder method.
    /// See the [core module's documentation](super::super#finders) for more information.
    pub fn unique_candidate(&self, grid_idx: GridIdx) -> Result<Option<Num>, CandidateError> {
        let sections = self.get_sections(grid_idx);
        let inner_idxs = grid_idx.inner_idxs();

        let mut result = Ok(None);

        for (section, inner_idx) in sections.into_iter().zip(inner_idxs) {
            let section_result = section.unique_candidate(inner_idx);
            match (result, section_result) {
                (_, Ok(None)) => (),
                (Ok(None), Ok(Some(_))) => result = section_result,
                (Ok(Some(a)), Ok(Some(b))) if a == b => (),
                _ => return Err(CandidateError::MultipleUniqueCandidates),
            }
        }

        result
    }
}
