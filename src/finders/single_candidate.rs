use crate::structs::{cell::Cell, num::Num};

/// [`Finder`](crate::finders)
impl Cell {
    /// Checks if a `Cell` is not known and only contains a single candidate.
    /// # Safety:
    /// Does not check that the underyling `u16` representation is correct.
    /// Any invalid `Cell` representation will return `false`.
    pub fn single_candidate(self) -> Option<Num> {
        let is_single_candidate = !self.is_known() && self.to_u16().count_ones() == 1;
        is_single_candidate.then_some(unsafe { Num::new_unchecked(self.to_u16().ilog2() as u8) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Standard usecase
    fn basic() {
        let cell = Cell::new(0b000000_001_001_001_0).unwrap();
        assert!(cell.single_candidate().is_none());

        let cell = Cell::new(0b000000_000_010_000_0).unwrap();
        assert_eq!(cell.single_candidate(), Some(Num::new(5).unwrap()));
    }

    #[test]
    // Reject known cells, even in banned representations
    fn discard_known() {
        let cell = Cell::new(0b000000_000_010_000_1).unwrap();
        assert!(cell.single_candidate().is_none());

        let cell = unsafe { Cell::new_unchecked(0b000000_000_111_000_1) };
        assert!(cell.single_candidate().is_none());
    }

    #[test]
    // Ignores banned bits
    fn ignore_banned_bits() {
        let cell = unsafe { Cell::new_unchecked(0b111000_001_001_001_0) };
        assert!(cell.single_candidate().is_none());

        let cell = unsafe { Cell::new_unchecked(0b111000_000_010_000_0) };
        assert_eq!(cell.single_candidate(), Some(Num::new(5).unwrap()));
    }
}
