use crate::structs::*;

impl Grid {
    /// Creates filters for all neighboring `Cell`s of a newly found known `Cell`
    /// # Comment
    /// Current implementation retains the 4 duplicates within the `Square` overlapped by the `Row`
    /// and `Column`
    /// # Safety
    /// The caller must ensure that the provided `Cell` at the given index is "known".
    /// Else the wrong `Filter`s will be created.
    pub fn get_known_filters(&mut self, _idx: GridIdx) -> [Filter; 24] {
        todo!("rewrite this using the new compliment_idx-less methods")
        /*
        let num = unsafe { self.get(idx).known_to_num() };
        let mask = num.to_neg_mask();

        let grid_idxs: [GridIdx; 24] = {
            let comp_idxs_nested = Grid::compliment_indices(idx);
            unsafe { transmute(comp_idxs_nested) }
        };

        let mut filters: [MaybeUninit<Filter>; 24] = MaybeUninit::uninit_array();

        for (idx, uninit_filter) in filters.iter_mut().enumerate() {
            *uninit_filter = MaybeUninit::new(Filter::new(mask, grid_idxs[idx]));
        }

        unsafe { MaybeUninit::array_assume_init(filters) }
        */
    }
}
