use crate::structs::*;
use std::mem::{transmute, MaybeUninit};

/// `Idx<N>` represents a number `x` which is guaranteed
/// to satisfy: `0 <= x < N`
#[derive(Clone, Copy)]
pub struct Idx<const N: u8>(u8);

impl<const N: u8> Idx<N> {
    // Create an Idx<N> from `n: u8`
    // Panics if `n >= N`
    pub fn new(n: impl Into<u8>) -> Self {
        let n: u8 = n.into();
        assert!(n < N);
        Self(n)
    }

    pub unsafe fn from_unchecked(value: impl Into<usize>) -> Self {
        Self::new(value.into() as u8)
    }
}

impl<const N: u8> TryFrom<u8> for Idx<N> {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < N {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl<const N: u8> From<Idx<N>> for u8 {
    fn from(value: Idx<N>) -> Self {
        value.0
    }
}

impl<const N: u8> From<Idx<N>> for usize {
    fn from(value: Idx<N>) -> Self {
        value.0 as usize
    }
}

pub fn pop(idxs: [GridIdx; 9], idx: SectionIdx) -> [GridIdx; 8] {
    // COMMENT: It seems like using maybeuninit has marginal assembly differences
    //          (https://godbolt.org/z/MbvrdTz6c) compared to simply instantiating
    //          the array manually.
    // COMMENT: Currently it is not allowed to do a transmute on a const len array
    //          So I can't make this function generic in the form:
    //          `fn([GridIdx; N], Idx<N>) -> [GridIdx; N - 1]`
    //          although its not clear if I'll ever even need it outside of N = 9.
    // COMMENT: This (^) is possible via `transmute_copy`, but one loses the edge over
    //          simply using an initialised array.
    let mut remaining_idxs: [MaybeUninit<GridIdx>; 8] =
        unsafe { MaybeUninit::uninit().assume_init() };

    let mut iter = idxs.into_iter();
    for (i, c) in remaining_idxs.iter_mut().enumerate() {
        let curr_idx = iter.next().unwrap();
        if i != idx.into() {
            *c = MaybeUninit::new(curr_idx);
        }
    }
    unsafe { transmute(remaining_idxs) }
}
