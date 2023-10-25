//! Unique Candidate:
//! When within a `Section`, a given `Cell` contains the only candidate for a certain `Num`.

use crate::{
    cell::{CandidateError, Cell},
    idx::{GridIdx, InnerIdx},
    num::Num,
    section::Section,
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
            _ => Err(CandidateError::KnownMultipleNum),
        }
    }
}
