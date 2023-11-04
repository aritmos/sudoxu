use super::super::{cell::CellMask, filter::Filter, idx::GridIdx};

use super::Grid;

impl Grid {
    /// Remove candidates at the specified location.
    pub fn apply_filter(&mut self, filter: Filter) {
        let cell = self.get_cell_mut(filter.idx);
        cell.remove_candidates(filter.mask);
    }

    /// Remove candidates at the specified locations in the [Grid].
    /// Applies [Grid::apply_filter](Grid::apply_filter) to each element in the given slice.
    pub fn apply_filters(&mut self, filters: &[Filter]) {
        for filter in filters {
            self.apply_filter(*filter);
        }
    }
}
