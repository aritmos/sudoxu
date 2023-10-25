//! Single candidate:
//! When a cell only contains one candidate.

use crate::cell::Cell;

impl Cell {
    /// Checks if a `Cell` is not known and only contains a single candidate.
    /// # Safety:
    /// Does not check that the underyling `u16` representation is correct.
    /// Any invalid `Cell` representation will return `false`.
    pub fn single_candidate(&self) -> bool {
        !self.is_known() && (self.to_u16() & 0b000000_111_111_111_0).count_ones() == 1
    }
}
