use super::section::{SectionInfo, SectionKind::*};

use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

/// A generic indexing type used to safely access comptime known-length slices and sets.
///
/// Backed by a `usize` `x` guaranteed to satisfy `x < N`.
#[derive(Clone, Copy, PartialEq)]
pub struct Idx<const N: usize>(usize);

/// Generic Implementations
impl<const N: usize> Idx<N> {
    /// Tries to create an `Idx<N>` from a `uint`.
    /// Returns None if the number is too large.
    pub fn new<T: Into<usize>>(t: T) -> Option<Self> {
        let n = t.into();
        if n < N {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Create an `Idx<N>` from a `usize`.
    ///
    /// Does not do the bounds check.
    /// # Safety
    /// Caller guarantees that `t.into() < N`.
    pub unsafe fn new_unchecked<T: Into<usize>>(t: T) -> Self {
        Self(t.into())
    }
}

impl<const N: usize> From<Idx<N>> for usize {
    /// [`Idx`] -> usize
    /// Used for index value computations.
    fn from(value: Idx<N>) -> Self {
        value.0
    }
}

// `x.to_idx()` ergonomics for casting `uint -> Idx`.

/// Safe <code>uint -> [Idx]</code> conversion.
pub trait TryIntoIdx: Into<usize> {
    /// Tries to cast a uint into an `Idx<N>`. Returns `None` if `self` was too large for `Idx<N>`
    /// (`n >= N`).
    fn try_to_idx<const N: usize>(self) -> Option<Idx<N>> {
        let n = self.into();
        (n < N).then_some(Idx::<N>(n))
    }
}

/// Unsafe <code>uint -> [Idx]</code> conversion.
pub trait IntoIdx: Into<usize> {
    /// Casts a uint into an `Idx<N>`, without performing the bounds check.
    /// # Safety
    /// The caller ensures that `self < N`.
    unsafe fn to_idx<const N: usize>(self) -> Idx<N> {
        Idx::<N>(self.into())
    }
}

impl IntoIdx for usize {}
impl TryIntoIdx for usize {}

// For indexing into slices without the SliceIndex trait for now
// impl<const N: u8> From<Idx<N>> for usize {
//     fn from(value: Idx<N>) -> Self {
//         value.0 as usize
//     }
// }
impl<T, const N: usize> Index<Idx<N>> for [T; N] {
    type Output = T;

    fn index(&self, index: Idx<N>) -> &Self::Output {
        unsafe { self.get_unchecked(index.0) }
    }
}

impl<T, const N: usize> IndexMut<Idx<N>> for [T; N] {
    fn index_mut(&mut self, index: Idx<N>) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index.0) }
    }
}

// ========================================================
// Concrete Indexes
// ========================================================

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
/// [`GridIdx`] Implementations
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

/// An index into a [`Section`](super::section::Section)'s inner
/// [Cells](super::cell::Cell).
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

/// [`AreaIdx`] Implementations
impl AreaIdx {
    /// Returns the three [`SectionIdxs`](SectionIdx) of the three boxes that make up the `Area` at
    /// the given index.
    pub fn to_box_section_idxs(self) -> [SectionIdx; 3] {
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
        BOX_SECTION_IDXS[usize::from(self)]
    }
}

// ========================================================
// Formatting
// ========================================================

impl<const N: usize> Display for Idx<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const N: usize> Debug for Idx<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let row_idx = SectionIdx::new(8_usize).unwrap();
        assert_eq!(row_idx.to_string(), "8");

        let grid_idx = GridIdx::new(8_usize).unwrap();
        assert_eq!(grid_idx.to_string(), "8");

        let grid_idx = GridIdx::new(60_usize).unwrap();
        assert_eq!(grid_idx.to_string(), "60");
    }
}
