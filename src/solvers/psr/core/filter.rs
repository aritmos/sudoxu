use super::{cell::CellMask, idx::GridIdx};

/// A [`CellMask`] and [`GridIdx`] pair. Used for removing candidates from the
/// [`Grid`](super::grid::Grid) using [`Grid::apply_filter`](super::grid::Grid::apply_filter).
#[derive(Clone, Copy)]
pub struct Filter {
    /// The [`CellMask`] to apply.
    pub mask: CellMask,
    /// The `Cell`'s [`GridIdx`] where the mask should be applied.
    pub idx: GridIdx,
}

impl Filter {
    /// Creates a new [`Filter`].
    pub fn new(mask: CellMask, idx: GridIdx) -> Self {
        Self { mask, idx }
    }
}
