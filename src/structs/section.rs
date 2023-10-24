use super::{
    cell::Cell,
    grid::Grid,
    idx::{GridIdx, SectionIdx},
};

use std::mem::MaybeUninit;

pub enum SectionKind {
    Row,
    Column,
    Box,
}

// Expose the enum variants for easier readability
pub use SectionKind::{Box, Column, Row};

// # Comment:
// This const param definition allows for generic and specific implementations;
// as well as the more explicit nature of the Section type being part of the type
// ```rust
// impl<const K: SectionKind> Section<K> {/**/}
// impl Section<{ Row }> {/**/}
// ```
/// A Section of the Grid, i.e. a Row, Column or Square.
/// # Note
/// Current implementation is generic over `u8`.
/// The correct implementation would be to be generic over `SectionIdx`,
/// but this is currently within the incomplete `adt_const_params` feature.
/// # Safety
/// `Section`'s generic `K` should only ever be constructed from `SectionIdx`:
/// by writing `Section::<{Row as u8}>`, etc.
#[derive(Debug)]
pub struct Section<const K: u8> {
    // idx: SectionIdx,
    pub cells: [Cell; 9],
}

impl<const K: u8> Section<K> {
    pub fn new(grid: &Grid, section_idx: SectionIdx) -> Section<K> {
        let grid_idxs = Grid::get_section_grididxs(Row, section_idx);
        let mut uninit_cells: [MaybeUninit<Cell>; 9] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (cell, grid_idx) in uninit_cells.iter_mut().zip(grid_idxs) {
            *cell = MaybeUninit::new(grid.get(grid_idx));
        }
        Section::<K> {
            cells: unsafe { std::mem::transmute(uninit_cells) },
        }
    }
}
