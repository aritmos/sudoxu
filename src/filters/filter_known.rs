use crate::structs::*;
use std::mem::transmute;

impl Cell {
    /// Returns `Some(n)` if the cell has a known `Num` "`n`" else None.
    /// # Safety
    /// Does no error checking to verify cell is correct.
    /// Simply looks at the "known" byte and counts ones.
    pub fn known(&self) -> Option<Num> {
        let cell_u16 = self.to_u16();
        if cell_u16 % 2 != 1 {
            return None;
        }
        if cell_u16.count_ones() != 2 {
            return None;
        }

        Some(unsafe { Num::new_unchecked(cell_u16.ilog2() as u8) })
    }
}

impl Grid {
    pub fn known(&mut self, idx: GridIdx) -> Filter {
        let Some(num) = self.get(idx).known() else {
            panic!("Tried to use `filter_known` on an unknown cell.");
        };
        let idxs = {
            let comp_idxs_nested = Grid::compliment_indices(idx);
            let comp_idxs_flat = unsafe { transmute::<_, [Idx<81>; 24]>(comp_idxs_nested) };
            Vec::from(comp_idxs_flat)
        };

        Filter::new(num.to_mask(), idxs)
    }
}
