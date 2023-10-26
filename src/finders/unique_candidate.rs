//! Unique Candidate:
//! When within a `Section`, a given `Cell` contains the only candidate for a certain `Num`.

use crate::{
    cell::{CandidateError, Cell},
    grid::Grid,
    idx::{GridIdx, InnerIdx},
    num::Num,
    section::{Section, SectionInfo, SectionKind::*},
};

impl Section {
    /// Finds if the `Cell` at `inner_idx` contains a candidate found in no
    /// other cells within the `Section`.
    /// # Safety:
    /// Does not check that the inner `Cell`s have correct representation.
    /// Incorrect `Cell` representations will lead to undefined behaviour.
    pub fn unique_candidate(&self, inner_idx: InnerIdx) -> Result<Option<Num>, CandidateError> {
        let mut cells = self.cells;
        let candidate_cell = cells[inner_idx];

        // OR all of the remaining candidates together
        cells[inner_idx] = unsafe { Cell::zerod() };
        let combined_candidates = cells.iter().fold(unsafe { Cell::zerod() }, |a, b| unsafe {
            Cell::new_unchecked(a.to_u16() | b.to_u16())
        });

        let unique_candidates_u16 =
            // Find bits set in candidate_cell but not in combined_candidates.
            (candidate_cell.to_u16() & !combined_candidates.to_u16());

        match unique_candidates_u16.count_ones() {
            0 => Ok(None),
            1 => {
                let n = unique_candidates_u16.ilog2() as u8;
                let num = unsafe { Num::new_unchecked(n) };
                Ok(Some(num))
            }
            _ => Err(CandidateError::MultipleUniqueCandidates),
        }
    }
}

impl Grid {
    /// Checks if a `Cell` contains a unique candidate within its selected `Section`s.
    ///
    /// `sections`' `[bool; 3]` represents selecting the `Cell`'s row, column and box
    /// respectively to be tested against for unique candidates.
    pub fn unique_candidate(&self, grid_idx: GridIdx) -> Result<Option<Num>, CandidateError> {
        let idx_info = grid_idx.associated_idxs();

        let mut result = Ok(None);

        for (section_kind, (section_idx, inner_idx)) in [Row, Column, Box].into_iter().zip(idx_info)
        {
            let section_info = SectionInfo::new(section_kind, section_idx);
            let section = self.section(section_info);
            let section_result = section.unique_candidate(inner_idx);
            match (result, section_result) {
                (_, Ok(None)) => (),
                (Ok(None), Ok(Some(_))) => result = section_result,
                (Ok(Some(a)), Ok(Some(b))) if a == b => (),
                _ => return Err(CandidateError::MultipleUniqueCandidates),
            }
        }

        result
    }
}
