use std::ops::{Index, IndexMut};

/// A number `x` guaranteed to satisfy `x < N`.
/// Used for indexing into arrays without fearing for out-of-bounds indexing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Idx<const N: usize>(usize);

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

/// An index into a `Section`'s Cells.
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

impl<const N: usize> Idx<N> {
    /// Tries to create an `Idx<N>` from a _uint_.
    /// # Note
    /// This is effectively a direct implementation of
    /// `impl<T: Into<usize>, const N: usize> TryFrom<T> for Idx<N>`,
    /// done to avoid the need to cover the type parameters.
    /// As such, while the function signature would be cleaner as an Option,
    /// it has been made to match the return of a `TryFrom` implementation.
    pub fn try_from<T: Into<usize>>(t: T) -> Result<Self, ()> {
        let n = t.into();
        if n < N {
            Ok(Self(n))
        } else {
            Err(())
        }
    }

    /// Create an `Idx<N>` from a `usize`
    /// TODO
    pub unsafe fn from_unchecked<T: Into<usize>>(t: T) -> Self {
        Self(t.into())
    }
}

// Idx -> usize
// For index value computation
impl<const N: usize> From<Idx<N>> for usize {
    fn from(value: Idx<N>) -> Self {
        value.0
    }
}

/// Safe "uint" -> Idx
pub trait TryIntoIdx: Into<usize> {
    /// Tries to cast a uint into an `Idx<N>`. Returns `None` if `self` was too large for `Idx<N>`
    /// (`n >= N`).
    fn try_to_idx<const N: usize>(self) -> Option<Idx<N>> {
        let n = self.into();
        if n < N {
            Some(Idx::<N>(n))
        } else {
            None
        }
    }
}

/// uint -> Idx
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
