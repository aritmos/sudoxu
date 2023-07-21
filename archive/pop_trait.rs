//! Archived due to not being an improvement over the const `array_pop` implementation.
//! Technically one should test the entire implementation chain, all the way to `unique_candidate`
//! Criterion Benchmarks:
/*
Pop/array_pop           time:   [1.1490 ns 1.1521 ns 1.1554 ns]
                        change: [-2.4136% -1.3723% -0.2553%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
Pop/(generic) trait     time:   [1.2771 ns 1.2808 ns 1.2849 ns]
                        change: [-0.7615% +0.1696% +1.0695%] (p = 0.73 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high sever
*/

// an iterator that skips its nth element
pub struct Pop<I> {
    iter: I,
    pop_idx: usize,
    curr_idx: usize,
}

impl<I: Iterator> Iterator for Pop<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_idx == self.pop_idx {
            self.iter.next();
        }
        self.curr_idx += 1;
        self.iter.next()
    }
}

pub trait PopIterator: IntoIterator {
    fn pop(self, n: usize) -> (Option<Self::Item>, Pop<Self::IntoIter>)
    where
        Self: Sized + Clone,
    {
        (
            self.clone().into_iter().nth(n),
            Pop {
                iter: self.into_iter(),
                pop_idx: n,
                curr_idx: 0,
            },
        )
    }
}

impl<I: IntoIterator> PopIterator for I {}

// this implementation is somehow even slower than the blanket implementation
// even when trying to hardcode an always in-bound case by setting the popped element to
// `unsafe {Some(self.get_unchecked(n))}`
impl PopIterator for &[Cell] {
    fn pop(self, n: usize) -> (Option<Self::Item>, Pop<Self::IntoIter>) {
        (
            self.get(n),
            Pop {
                iter: self.iter(),
                pop_idx: n,
                curr_idx: 0,
            },
        )
    }
}
