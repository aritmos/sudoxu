#![allow(unused)]
use crate::structs::*;

/// Unique Candidate:
/// When within a row, column or square only one cell can contain a given number;

impl Cell {
    fn unique_candidate(&self, arr: &[Cell]) -> Result<Option<Num>, CellError> {
        // combine all other candidate cells
        let combined_candidates = arr
            .iter()
            .fold(unsafe { Self::new_unchecked(0) }, |a, b| a | *b)
            & Self::default();

        // find which bits only exist in `self`
        let sole_candidates = ((*self ^ combined_candidates) & *self).to_u16();

        match sole_candidates.count_ones() {
            0 => Ok(None),
            1 => {
                let n = sole_candidates.ilog2() as u8;
                Ok(Some(unsafe { Num::new_unchecked(n) }))
            }
            2.. => Err(CellError::MultipleSoleCandidates),
        }
    }
}

impl Grid {
    pub fn unique_candidate(&self, idx: GridIdx) -> CellResult {
        // Get the cells that we will test against
        let cell = self.get(idx);
        let [row_comp_idxs, col_comp_idxs, square_comp_idxs] = Grid::compliment_indices(idx);

        let row_comp_cells = self.get_cells(row_comp_idxs);
        let col_comp_cells = self.get_cells(col_comp_idxs);
        let square_comp_cells = self.get_cells(square_comp_idxs);

        // Test each section for a unique candidate
        let section_results = [
            cell.unique_candidate(&row_comp_cells),
            cell.unique_candidate(&col_comp_cells),
            cell.unique_candidate(&square_comp_cells),
        ];

        // Fold the results of each section's unique_candidate results into one
        // TODO: how to properly deal with sections returning multiple errors
        // CURRENT IMPL: returns first error it encounters in the order: row, col, square.
        section_results
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
