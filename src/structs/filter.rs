use crate::structs::*;

/// Type definition to better showcase the intent of a `Cell`
/// in certain contexts.
pub type Mask = Cell;

pub struct Filter {
    mask: Mask,
    idxs: Vec<GridIdx>,
}

impl Num {
    pub fn to_mask(self) -> Mask {
        unsafe { Mask::new_unchecked(!(1 << self as u8)) }
    }
}

impl Filter {
    pub fn new(mask: Mask, idxs: Vec<GridIdx>) -> Self {
        Self { mask, idxs }
    }
}

impl Grid {
    pub fn filter(&mut self, filter: Filter) {
        for idx in filter.idxs {
            let cell = self.get_mut(idx);
            cell.mask(filter.mask);
        }
    }

    pub fn filter_multiple(&mut self, filters: Vec<Filter>) {
        for filter in filters {
            self.filter(filter);
        }
    }
}

impl Cell {
    pub fn mask(&mut self, mask: Mask) {
        *self &= mask
    }
}
