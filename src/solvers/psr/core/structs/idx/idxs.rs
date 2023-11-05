//! Concrete [`Idx`]s.
use super::Idx;

use super::super::section::{SectionInfo, SectionKind::*};

/// An index into the cells of a grid:
/// ```txt
/// 00 01 02 │ 03 04 05 │ 06 07 08
/// 09 10 11 │ 12 13 14 │ 15 16 17
/// 18 19 20 │ 21 22 23 │ 24 25 26
/// ─────────┼──────────┼─────────
/// 27 28 29 │ 30 31 32 │ 33 34 35
/// 36 37 38 │ 39 40 41 │ 42 43 44
/// 45 46 47 │ 48 49 50 │ 51 52 53
/// ─────────┼──────────┼─────────
/// 54 55 56 │ 57 58 59 │ 60 61 62
/// 63 64 65 │ 66 67 68 │ 69 70 71
/// 72 73 74 │ 75 76 77 │ 78 79 80
/// ```
pub type GridIdx = Idx<81>;

// Normally we would wrap `Idx<81>` into its own type when introducing implementations.
// However, in this case we know the only use of `Idx<81>` will be within `GridIdx`.
// Hence we somewhat sloppily use the type alias to showcase intent, and implement directly on the
// `Idx` type.
impl GridIdx {
    /// Returns the `SectionIdx`s associated to the given `GridIdx`, returned in the order: Row,
    /// Column, Box.
    /// # (Simplified) Examples
    /// ```txt
    /// 45 => [5, 0, 3]
    /// ```
    pub fn section_idxs(self) -> [SectionIdx; 3] {
        let i = self.0;
        let row = i / 9;
        let col = i % 9;
        let box_ = 3 * (row / 3) + (col / 3);
        unsafe { std::mem::transmute([row, col, box_]) }
    }

    /// Returns the `InnerIdx`s associated to the given `GridIdx`, returned in the order: Row,
    /// Column, Box.
    /// # (Simplified) Examples
    /// ```txt
    /// 45 => [0, 5, 6]
    /// ```
    pub fn inner_idxs(&self) -> [InnerIdx; 3] {
        let i = self.0;
        let row = i / 9;
        let col = i % 9;
        let box_inner = 3 * (row % 3) + (col % 3);
        unsafe { std::mem::transmute([col, row, box_inner]) }
    }
}

/// An index into the sections of a grid.
/// For rows and columns, these are their own row and column number (zero indexed) starting from
/// the top and left.
/// For boxes these are:
/// ```txt
/// 0 │ 1 │ 2
/// ──┼───┼──
/// 3 │ 4 │ 5
/// ──┼───┼──
/// 6 │ 7 │ 8
/// ```
pub type SectionIdx = Idx<9>;

/// An index into a [`Section`](super::super::section::Section)'s inner
/// [Cells](super::super::cell::Cell).
/// For rows(columns) these are their cell's column(row) `SectionIdx`.
/// For boxes these are:
/// ```txt
/// 0 1 2 │ 0 1 2 │ 0 1 2
/// 3 4 5 │ 3 4 5 │ 3 4 5
/// 6 7 8 │ 6 7 8 │ 6 7 8
/// ──────┼───────┼──────
/// 0 1 2 │  ...  │
/// ```
pub type InnerIdx = Idx<9>;

// TODO: define an Area in the documentation as it doesnt exist as a standalone struct
/// An index into the `Area`s of the grid.
/// ```txt
///  0   1   2
///  ↓   ↓   ↓
///    │   │   <- 3
/// ───┼───┼───
///    │   │   <- 4
/// ───┼───┼───
///    │   │   <- 5
/// ```
pub type AreaIdx = Idx<6>;
