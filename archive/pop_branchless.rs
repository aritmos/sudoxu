//! Branchless `array_pop` implementation. Discarded for performance over the current `for loop`
//! implementation.
//! # Benchmark
/*
Pop/For Loop            time:   [3.3064 ns 3.3197 ns 3.3405 ns]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
Pop/Slice Copy          time:   [11.148 ns 11.177 ns 11.220 ns]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high sever
*/
//! # Code

/// Pops the `n`th element from the array and returns the remaining elements in a new array
/// COMMENT: branchless implementation
pub fn array_pop_bl<const N: usize, T>(idxs: [T; N], n: Idx<{ N as u8 }>) -> [T; N - 1]
where
    T: Copy,
{
    let mut remaining: [MaybeUninit<T>; N - 1] = unsafe { MaybeUninit::uninit().assume_init() };

    let n = usize::from(n);

    MaybeUninit::write_slice(&mut remaining[..n], &idxs[..n]);
    MaybeUninit::write_slice(&mut remaining[n..], &idxs[n + 1..]);

    unsafe { MaybeUninit::array_assume_init(remaining) }
}
