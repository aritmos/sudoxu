#![allow(unused)]
use crate::structs::*;

/// Inner index within a `Section`
type InnerIdx = Idx<9>;

/// Unique Candidate:
/// When within a `Section`, a specific `Cell` contains the only candidate for a certain `Num`.
impl<const K: SectionKind> Section<K> {
    fn unique_candidate(self, inner_idx: InnerIdx) -> CellResult {
        let (candidate_cell, remaining_cells) = {
            let mut cells = self.cells;
            let cell = cells[inner_idx];
            cells[inner_idx] = unsafe { Cell::zero() };
            (cell, cells)
        };
        let combined_candidates = remaining_cells
            .iter()
            .fold(unsafe { Cell::zero() }, |a, b| a | *b);

        let sole_candidates = ((candidate_cell ^ combined_candidates) & candidate_cell).to_u16();

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
        todo!("rewrite to remove compliment indices");
        /*
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
        */
    }
}
