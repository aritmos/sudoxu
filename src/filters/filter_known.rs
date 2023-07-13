use crate::structs::*;
use std::mem::{transmute, MaybeUninit};

impl Cell {
    /// Returns `Some(n)` if the cell has a known `Num` "`n`" else None.
    /// # Safety
    /// Does no error checking to verify cell is correct.
    /// Simply looks at the "known" byte and counts toggled bits.
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
    /// Creates filters for all neighboring `Cell`s of a newly found known `Cell`
    /// # Safety
    /// The caller must ensure that the provided `Cell` at the given index is "known"
    /// Panics if not a known `Cell`
    pub fn get_known_filters(&mut self, idx: GridIdx) -> [Filter; 24] {
        let num = self.get(idx).known().unwrap();
        let mask = num.to_neg_mask();

        let idxs: [GridIdx; 24] = {
            let comp_idxs_nested = Grid::compliment_indices(idx);
            unsafe { transmute(comp_idxs_nested) }
        };

        let mut filters: [MaybeUninit<Filter>; 24] = MaybeUninit::uninit_array();

        for (idx, uninit_filter) in filters.iter_mut().enumerate() {
            *uninit_filter = MaybeUninit::new(Filter::new(mask, idxs[idx]));
        }

        unsafe { MaybeUninit::array_assume_init(filters) }
    }
}
