use crate::structs::*;
use std::mem::{transmute, MaybeUninit};

impl Area {
    /// Checks if there are any candidates that only lie in a row
    /// then removes said candidate from said row in the other squares
    /// within the masks.
    pub fn single_line(&mut self) {
        let candidate_lines: [Cell; 3] = {
            let mut uninit_unknown_count_1s: [MaybeUninit<Cell>; 3] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for (i, cell) in uninit_unknown_count_1s.iter_mut().enumerate() {
                *cell = MaybeUninit::new(self.count_1[i] & !self.known[i]);
            }
            unsafe { transmute(uninit_unknown_count_1s) }
        };

        for (idx, candidate_line) in candidate_lines.iter().enumerate() {
            // skip if there are no candidates
            if *candidate_line == !Cell::default() {
                continue;
            }

            fn compliment_idxs(idx: Idx<3>) -> [Idx<3>; 2] {
                unsafe {
                    transmute(match u8::from(idx) {
                        0 => [1_u8, 2],
                        1 => [0, 2],
                        2 => [0, 1],
                        _ => unreachable!(),
                    })
                }
            }

            let subsection_compliment_idxs = compliment_idxs(unsafe { Idx::from_unchecked(idx) });

            for num in 1..=9 {
                let num = Num::new_unchecked(num);

                // skip if num is not one of the candidates
                if !candidate_line.contains_candidate(num) {
                    continue;
                }

                // find at which row index in the subsection we have the `1`
                let row_idx = unsafe {
                    Idx::<3>::from_unchecked(
                        <[Cell; 3]>::from(self.values[idx])
                            .iter()
                            .position(|c| c.contains_candidate(num))
                            .unwrap(),
                    )
                };

                let row_compliment_idxs = compliment_idxs(row_idx);

                // apply masks
                let mask = unsafe { Cell::new_unchecked(!(1 << num as u8)) };
                for i in subsection_compliment_idxs {
                    for j in row_compliment_idxs {
                        self.masks[usize::from(i)][usize::from(j)] &= mask;
                    }
                }
            }
        }
    }

    /// Checks if there are any candidates that only lie in the same two rows
    /// within different squares. Then removes said candidate from the remaining
    /// row in the remaining square within the masks.
    pub fn double_lines(&mut self) {
        todo!()
    }
}
