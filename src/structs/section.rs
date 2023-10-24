use super::{
    cell::Cell,
};

#[derive(Debug)]
pub struct Section {
    kind: SectionKind,
    cells: [Cell; 9]
}

#[derive(Debug)]
pub enum SectionKind {
    Row = 0,
    Column = 1,
    Box = 2,
}

impl Section {
    pub fn new(kind: SectionKind, cells: [Cell; 9]) -> Self {
        Self {kind, cells}
    }
}
