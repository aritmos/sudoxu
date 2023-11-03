use crate::solvers::psr::core::cell::CellMask;

use super::super::{
    filter::Filter,
    grid::Grid,
    idx::GridIdx,
    num::Num,
    section::{SectionInfo, SectionKind::*},
};

impl Grid {
    /// Creates filters for the neighbours of a cell when its value is found.
    /// The [`Num`] and [`GridIdx`] passed select the [`Cell`](super::super::cell::Cell) and its value.
    ///
    /// # Note
    /// Technically there are only 20 [`Cells`](super::super::cell::Cell) to update when a known
    /// [`Cell`](super::super::cell::Cell) is found due the overlap of
    /// [`Sections`](super::super::section::Section). However the overhead of removing these
    /// duplicates in the current implementation is greater than keeping them and filtering four
    /// [`Cells`](super::super::cell::Cell) twice.
    pub fn known_filters(n: Num, grid_idx: GridIdx) -> [Filter; 24] {
        use std::mem::MaybeUninit;

        let section_idxs = grid_idx.section_idxs();
        let section_infos = [
            SectionInfo::new(Row, section_idxs[0]),
            SectionInfo::new(Column, section_idxs[1]),
            SectionInfo::new(Box, section_idxs[2]),
        ];

        let sections_grididxs = section_infos.map(|si| si.grid_idxs());
        let inner_idxs = grid_idx.inner_idxs();

        let mask = CellMask::from_known(n);
        let mut write_idx = 0;
        let mut filters: [MaybeUninit<Filter>; 24] = unsafe { MaybeUninit::uninit().assume_init() };

        for (section_grididxs, inner_idx) in sections_grididxs.into_iter().zip(inner_idxs) {
            for (i, grid_idx) in section_grididxs.into_iter().enumerate() {
                // skip the known cell
                if i == usize::from(inner_idx) {
                    continue;
                }

                let filter = Filter::new(mask, grid_idx);
                filters[write_idx] = MaybeUninit::new(filter);
                write_idx += 1;
            }
        }

        unsafe { std::mem::transmute(filters) }
    }
}
