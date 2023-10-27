//! Updating Cells

use super::Cell;
use crate::structs::num::Num;

/// [`Cell`] Modification
impl Cell {
    /// Sets the known bit of the given cell.
    /// # Safety
    /// Caller guarantees that the cell only has a single candidate,
    /// such that setting the known bit results in a valid representation.
    pub unsafe fn set_known(&mut self) {
        self.0 |= 1;
    }

    /// Adds the set candidate bits in the `CellMask` to the `Cell`.
    /// # Safety
    /// This function is provided to give a performance improvement to candidate removal.
    /// The caller guarantees that this operation respects the `Cell` representation.
    pub unsafe fn set_candidates(&mut self, mask: CellMask) {
        self.0 |= mask.0
    }

    /// Removes the candidate bits set in the mask from the cell.
    pub fn remove_candidates(&mut self, mask: CellMask) {
        self.0 &= !mask.0
    }
}

/// Used to remove multiple candidates from a `Cell`.
/// Will always be applied as a negative mask onto the candidate bits.
pub struct CellMask(u16);

impl CellMask {
    pub fn from_known(num: Num) -> Self {
        Self(1 << u16::from(num))
    }

    pub fn from_candidates(nums: &[Num]) -> Self {
        let mut x = 0_u16;
        for &num in nums {
            x |= 1 << u16::from(num);
        }
        Self(x)
    }
}
