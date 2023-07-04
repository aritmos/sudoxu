use crate::structs::*;
use std::mem::{transmute, MaybeUninit};

impl Area {
    /// Checks if there are any candidates that only lie in a row
    /// then removes said candidate from said row in the other squares
    /// within the masks.
    pub fn single_line(&mut self) {
        // count_1 but having filtered known `Num`
        // (exactly what we need for this technique)
        let candidate_lines: [Cell; 3] = {
            let count_1 = self.get_count::<1>();
            let known = self.get_known();

            let mut uninit_unknown_count_1s: [MaybeUninit<Cell>; 3] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for (i, cell) in uninit_unknown_count_1s.iter_mut().enumerate() {
                *cell = MaybeUninit::new(count_1[i] & !known[i]);
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

            let subsection_compliment_idxs =
                compliment_idxs(unsafe { Idx::new_unchecked(idx as u8) });

            for num in 1..=9 {
                let num = unsafe { Num::new_unchecked(num) };

                // skip if num is not one of the candidates
                if !candidate_line.contains_candidate(num) {
                    continue;
                }

                // find at which row index in the subsection we have the `1`
                let row_idx = unsafe {
                    Idx::<3>::new_unchecked(
                        <[Cell; 3]>::from(self.values[idx])
                            .iter()
                            .position(|c| c.contains_candidate(num))
                            .unwrap() as u8,
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
        // this is possible because the ordering of matches makes it such that
        // `self.matches[i]` contains the info of the ith square
        // (aka it aligns with the data in `self.count_2`)

        let contains_2_matches: [Cell; 3] = {
            let matches = self.get_matches();
            let contains_2 = self.get_count::<2>();
            // we need to filter the matches to those which only also "contain_2"
            // we do this by `AND`ing with the `contains_2` of either square in the pair
            [
                matches[0] & contains_2[0], // 0 <-> 1 contains 0
                matches[1] & contains_2[0], // 0 <-> 2 contains 0
                matches[2] & contains_2[1], // 1 <-> 2 contains 1
            ]
        };

        for (idx, contains_2_match) in contains_2_matches.into_iter().enumerate() {
            // skip to next pair if there are no matching candidates
            if contains_2_match == unsafe { Cell::new_unchecked(0) } {
                continue;
            }

            for num in 1..=9 {
                let num = unsafe { Num::new_unchecked(num) };

                // skip if num is not one of the candidates
                if !contains_2_match.contains_candidate(num) {
                    continue;
                }

                // find at which row index in the subsection we have the `0`
                let row_idx = unsafe {
                    Idx::<3>::new_unchecked(
                        <[Cell; 3]>::from(self.values[idx])
                            .iter()
                            .position(|c| !c.contains_candidate(num))
                            .unwrap() as u8,
                    )
                };

                // apply masks
                let mask = unsafe { Mask::new_unchecked(!(1 << num as u8)) };
                // the masked square index is given by `2-idx`: [0-1, 0-2, 1-2] => [2, 1, 0]
                self.masks[2 - idx][usize::from(row_idx)] &= mask;
            }
        }
    }
}
