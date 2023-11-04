use super::{cell::CellMask, idx::GridIdx};

/// A [`CellMask`] and [`GridIdx`] pair. Used for removing candidates from the
/// [`Grid`](super::grid::Grid) using [`Grid::apply_filter`](super::grid::Grid::apply_filter).
#[derive(Clone, Copy)]
pub struct Filter {
    pub mask: CellMask,
    pub idx: GridIdx,
}

impl Filter {
    pub fn new(mask: CellMask, idx: GridIdx) -> Self {
        Self { mask, idx }
    }
}
