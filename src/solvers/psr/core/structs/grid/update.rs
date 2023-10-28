use super::super::{cell::CandidateError, idx::GridIdx};

use super::Grid;

impl Grid {
    /// Update the neighbours (all remaining `Cell`s in each `Section`) once a `Cell` is known to have a value.
    pub fn known_cell_update_neighbours(&mut self, grid_idx: GridIdx) {
        todo!()
    }
}
