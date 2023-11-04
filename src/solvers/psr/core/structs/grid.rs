mod fmt;
#[doc(hidden)]
mod update;
#[doc(inline)]
pub use update::*;

use super::{
    cell::Cell,
    idx::GridIdx,
    section::{Section, SectionInfo, SectionKind},
};

#[derive(PartialEq, Eq)]
/// The sudoku board for PSR solvers.
///
/// #### Nomenclature
/// A [`Cell`](Cell)'s "neighbours" are all other [`Cells`](Cell) in the grid that share a [`Section`]
/// with the given [`Cell`].
///
/// #### Grid Correctness
/// At any given time in a PSR Solver's computation, [`Cells`](Cell) within the Grid may contain
/// candidates that are "not allowed" due to neighboring known cells. This is a valid representation.
/// In these cases said [`Cells`](Cell) might have not been "updated" upon finding a value for a cell.
/// Care must be taken to promptly update [`Cells`](Cell) every time candidates are filtered or
/// values are found.
pub struct Grid(pub(super) [Cell; 81]);

#[derive(Debug)]
/// Grid-related errors
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
    /// Get a copy of the [`Cell`] the given index.
    #[inline(always)]
    pub fn get_cell(&self, idx: GridIdx) -> Cell {
        self.0[idx]
    }

    /// Get a mutable reference to the [`Cell`] at the given index.
    pub fn get_cell_mut(&mut self, idx: GridIdx) -> &mut Cell {
        &mut self.0[idx]
    }
}

// Section Related
impl Grid {
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
}
