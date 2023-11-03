use super::{cell::CellMask, idx::GridIdx};

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
