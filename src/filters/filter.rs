use crate::structs::*;

/// Specifies that the candidate 'num'
/// should be removed from the cells in the contained indices

pub enum Filter {
    Num(Num),
    Mask(Cell),
}

pub struct GridFilter {
    filter: Filter,
    idxs: Vec<GridIdx>,
}

impl GridFilter {
    pub fn new(filter: Filter, idxs: Vec<GridIdx>) -> Self {
        Self { filter, idxs }
    }
}

impl Grid {
    pub fn filter(&mut self, grid_filter: GridFilter) {
        let filter = grid_filter.filter;
        let grid_idxs = grid_filter.idxs;

        for idx in grid_idxs {
            let mask = match filter {
                Filter::Num(n) => n.to_mask(),
                Filter::Mask(m) => m,
            };

            self.get_mut(idx).mask(mask);
        }
    }
}
