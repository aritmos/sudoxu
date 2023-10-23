use crate::structs::*;

/// Single Candidate:
/// When a cell can only contain one number

impl Cell {
    fn sole_candidate(&self) -> CellResult {
        let cell_u16 = self.to_u16();
        if cell_u16 == 0 {
            return Err(CellError::NoCandidates);
        }

        // same assembly if we use `is_power_of_two()`
        if cell_u16.count_ones() == 1 {
            let n = cell_u16.ilog2() as u8;
            let num = unsafe { Num::new_unchecked(n) };
            Ok(Some(num))
        } else {
            Ok(None)
        }
    }
}

impl Grid {
    pub fn sole_candidate(&self, idx: GridIdx) -> CellResult {
        let cell = self.get(idx);
        cell.sole_candidate()
    }
}
