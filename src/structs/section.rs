use super::{cell::Cell, idx::SectionIdx};

#[derive(Debug)]
pub struct Section {
    pub info: SectionInfo,
    pub cells: [Cell; 9],
}

#[derive(Debug, Copy, Clone)]
pub struct SectionInfo {
    pub kind: SectionKind,
    pub idx: SectionIdx,
}

#[derive(Debug, Copy, Clone)]
pub enum SectionKind {
    Row = 0,
    Column = 1,
    Box = 2,
}

impl Section {
    pub fn new(info: SectionInfo, cells: [Cell; 9]) -> Self {
        Self { info, cells }
    }
}

impl SectionInfo {
    pub fn new(kind: SectionKind, idx: SectionIdx) -> Self {
        Self { kind, idx }
    }
}
