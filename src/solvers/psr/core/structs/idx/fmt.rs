use super::Idx;

use std::fmt::{Debug, Display};

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
        use super::super::{GridIdx, SectionIdx};
        let row_idx = SectionIdx::new(8_usize).unwrap();
        assert_eq!(row_idx.to_string(), "8");

        let grid_idx = GridIdx::new(8_usize).unwrap();
        assert_eq!(grid_idx.to_string(), "8");

        let grid_idx = GridIdx::new(60_usize).unwrap();
        assert_eq!(grid_idx.to_string(), "60");
    }
}
