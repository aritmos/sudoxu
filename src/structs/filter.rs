use crate::structs::*;

/// Type definition to better showcase the intent of a `Cell`
/// in certain contexts.
pub type Mask = Cell;

#[derive(Clone, Copy)]
pub struct Filter {
    pub mask: Mask,
    pub idx: GridIdx,
}

impl Num {
    /// Creates a `Mask` from a `Num`.
    /// The `Mask` has `1`s except at the given index.
    /// # Example
    /// ```rust
    /// let n = Num::from_unchecked(4);
    /// let mask = n.to_mask();
    /// assert_eq!(mask, 0b000000_000_001_000_0);
    /// ```
    #[inline]
    pub fn to_mask(self) -> Mask {
        unsafe { Mask::new_unchecked(1 << self as u8) }
    }

    /// Creates a negative `Mask` from a `Num`.
    /// The `Mask` has `1`s except at the given index.
    /// # Example
    /// ```rust
    /// let n = Num::from_unchecked(4);
    /// let mask = n.to_neg_mask();
    /// assert_eq!(mask, 0b111111_111_110_111_1);
    /// ```
    /// # Safety
    /// The caller guarantees that the produced `Mask` is only used on `Cell`s
    /// via bitwise `AND`, such that prohibitted bits never get set to `1`.
    #[inline]
    pub fn to_neg_mask(self) -> Mask {
        unsafe { Mask::new_unchecked(!(1 << self as u8)) }
    }
}

impl Filter {
    pub fn new(mask: Mask, idx: GridIdx) -> Self {
        Self { mask, idx }
    }
}

impl Grid {
    pub fn filter(&mut self, filter: Filter) {
        let cell = self.get_mut(filter.idx);
        *cell &= filter.mask;
    }

    pub fn filter_multiple(&mut self, filters: &[Filter]) {
        for filter in filters {
            self.filter(*filter);
        }
    }
}
