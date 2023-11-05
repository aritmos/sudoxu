mod simd;
#[doc(inline)]
pub use simd::*;

use super::{
    cell::Cell,
    idx::{GridIdx, InnerIdx, SectionIdx},
};

#[derive(Debug, Clone)]
/// A row, column, or box of [`Cells`](Cell).
pub struct Section {
    /// The information linking to what [`Section`] within the
    /// [`Grid`](super::super::structs::grid::Grid) this section was taken from.
    pub info: SectionInfo,
    /// The underlying array of [`Cells`](Cell).
    pub cells: [Cell; 9],
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// Information to identify a Section within the grid.
pub struct SectionInfo {
    /// What kind of [`Section`] is it.
    pub kind: SectionKind,
    /// What (section) index is it.
    pub idx: SectionIdx,
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// An enum for representing the kinds of [`Sections`](Section).
pub enum SectionKind {
    /// Row: Horizontal line within the grid.
    Row = 0,
    /// Column: Vertical line within the grid.
    Column = 1,
    /// Box: 3x3 square within the grid.
    Box = 2,
}

impl Section {
    /// Create a new [`Section`].
    pub(super) fn new(info: SectionInfo, cells: [Cell; 9]) -> Self {
        Self { info, cells }
    }
}

impl SectionInfo {
    /// Create a new [`SectionInfo`].
    pub fn new(kind: SectionKind, idx: SectionIdx) -> Self {
        Self { kind, idx }
    }

    /// Return the grid indexes of the provided section.
    pub fn grid_idxs(self) -> [GridIdx; 9] {
        const SECTION_GRIDIDXS: [[usize; 9]; 27] = [
            // Rows
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24, 25, 26],
            [27, 28, 29, 30, 31, 32, 33, 34, 35],
            [36, 37, 38, 39, 40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49, 50, 51, 52, 53],
            [54, 55, 56, 57, 58, 59, 60, 61, 62],
            [63, 64, 65, 66, 67, 68, 69, 70, 71],
            [72, 73, 74, 75, 76, 77, 78, 79, 80],
            // Columns
            [0, 9, 18, 27, 36, 45, 54, 63, 72],
            [1, 10, 19, 28, 37, 46, 55, 64, 73],
            [2, 11, 20, 29, 38, 47, 56, 65, 74],
            [3, 12, 21, 30, 39, 48, 57, 66, 75],
            [4, 13, 22, 31, 40, 49, 58, 67, 76],
            [5, 14, 23, 32, 41, 50, 59, 68, 77],
            [6, 15, 24, 33, 42, 51, 60, 69, 78],
            [7, 16, 25, 34, 43, 52, 61, 70, 79],
            [8, 17, 26, 35, 44, 53, 62, 71, 80],
            // Boxes
            [0, 1, 2, 9, 10, 11, 18, 19, 20],
            [3, 4, 5, 12, 13, 14, 21, 22, 23],
            [6, 7, 8, 15, 16, 17, 24, 25, 26],
            [27, 28, 29, 36, 37, 38, 45, 46, 47],
            [30, 31, 32, 39, 40, 41, 48, 49, 50],
            [33, 34, 35, 42, 43, 44, 51, 52, 53],
            [54, 55, 56, 63, 64, 65, 72, 73, 74],
            [57, 58, 59, 66, 67, 68, 75, 76, 77],
            [60, 61, 62, 69, 70, 71, 78, 79, 80],
        ];
        let usize_section_grididxs =
            SECTION_GRIDIDXS[self.kind as usize * 9 + usize::from(self.idx)];
        unsafe { std::mem::transmute(usize_section_grididxs) }
    }
}
