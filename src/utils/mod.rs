use crate::structs::Idx;
use std::mem::MaybeUninit;

/// Pops the `idx`th element from the array, returning an array
/// # Signature
/// ```rust
/// pub fn pop<const N: usize, T>(idxs: [T; N], idx: Idx<{ N as u8 }>) -> [T; N - 1];
/// ```
pub fn pop<const N: usize, T>(idxs: [T; N], idx: Idx<{ N as u8 }>) -> [T; N - 1] {
    let mut remaining_idxs: [MaybeUninit<T>; N - 1] =
        unsafe { MaybeUninit::uninit().assume_init() };

    let mut iter = idxs.into_iter();
    for (i, c) in remaining_idxs.iter_mut().enumerate() {
        let curr_idx = iter.next().unwrap();
        if i != idx.into() {
            *c = MaybeUninit::new(curr_idx);
        }
    }
    unsafe { MaybeUninit::array_assume_init(remaining_idxs) }
}
