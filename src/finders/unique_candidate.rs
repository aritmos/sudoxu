#![allow(unused)]
use crate::structs::*;

/// Unique Candidate:
/// When within a row, column or square only one cell can contain a given number;

impl Cell {
    /// Finds if a `Cell` contains a unique candidate when compared against the given slice.
    /// Returns a `CellError::MultipleSoleCandidates` if there are multiple candidates.
    pub fn unique_candidate(self, arr: &[Cell]) -> CellResult {
        // combine all other candidate cells
        let combined_candidates = arr
            .iter()
            .fold(unsafe { Self::new_unchecked(0) }, |a, b| a | *b)
            & Self::default();

        // find which bits only exist in `self`
        let sole_candidates = ((self ^ combined_candidates) & self).to_u16();

        match sole_candidates.count_ones() {
            0 => Ok(None),
            1 => {
                let n = sole_candidates.ilog2() as u8;
                let num = unsafe { Num::new_unchecked(n) };
                Ok(Some(num))
            }
            2.. => Err(CellError::MultipleSoleCandidates),
        }
    }
}

impl Grid {
    /// Finds if a `Cell` contains a unique candidate when compared against its neighbours in a
    /// grid. i.e. apply `Cell::unique_candidate` within each `Section` containing the `Cell` and
    /// combine the results.
    pub fn unique_candidate(&self, idx: GridIdx) -> CellResult {
        let cell = self.get(idx);

        Grid::compliment_indices(idx)
            // Compliment cells for each section
            .map(|idxs| self.get_cells(idxs))
            // unique candidate result for each section
            .map(|comp_cells| cell.unique_candidate(&comp_cells))
            // Fold the results of each section's unique_candidate results into one
            // TODO: how to properly deal with sections returning multiple errors
            // CURRENT IMPL: returns first error it encounters in the order: row, col, square.
            .into_iter()
            .fold(Ok(None), |x, y| match (x, y) {
                (Err(e), _) | (_, Err(e)) => Err(e),
                (Ok(None), Ok(None)) => Ok(None),
                (Ok(Some(a)), Ok(None)) | (Ok(None), Ok(Some(a))) => Ok(Some(a)),
                (Ok(Some(a)), Ok(Some(b))) => {
                    if a == b {
                        Ok(Some(a))
                    } else {
                        Err(CellError::MultipleSoleCandidates)
                    }
                }
            })
    }
}
