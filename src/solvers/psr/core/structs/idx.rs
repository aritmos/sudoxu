pub mod idxs;

#[doc(inline)]
pub use idxs::*;
mod fmt;

use std::ops::{Index, IndexMut};

/// A generic indexing type used to safely access comptime known-length slices and sets.
///
/// Backed by a `usize` `x` guaranteed to satisfy `x < N`.
#[derive(Clone, Copy, PartialEq)]
pub struct Idx<const N: usize>(usize);

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
