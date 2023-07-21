use crate::structs::*;
use std::mem::MaybeUninit;

/// Pops the `n`th element from the array and returns the remaining elements in a new array
pub fn array_pop<const N: usize, T>(idxs: [T; N], n: Idx<{ N as u8 }>) -> [T; N - 1] {
    let mut remaining_idxs: [MaybeUninit<T>; N - 1] =
        unsafe { MaybeUninit::uninit().assume_init() };

    let mut iter = idxs.into_iter();
    for (i, c) in remaining_idxs.iter_mut().enumerate() {
        let curr_idx = iter.next().unwrap();
        if i != usize::from(n) {
            *c = MaybeUninit::new(curr_idx);
        }
    }
    unsafe { MaybeUninit::array_assume_init(remaining_idxs) }
}
