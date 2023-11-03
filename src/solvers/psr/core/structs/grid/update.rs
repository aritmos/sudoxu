use super::super::{cell::CellMask, filter::Filter, idx::GridIdx};

use super::Grid;

impl Grid {
    /// Remove candidates at the specified location.
    pub fn filter(&mut self, filter: Filter) {
        let cell = self.get_mut(filter.idx);
        cell.remove_candidates(filter.mask);
    }

    /// Remove candidates at the specified locations in the [Grid].
    /// Applies [Grid::filter](Grid::filter) to each element in the given slice.
    pub fn filter_multiple(&mut self, filters: &[Filter]) {
        for filter in filters {
            self.filter(*filter);
        }
    }
}
