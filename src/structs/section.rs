use crate::structs::*;
use std::fmt::Display;
use std::marker::ConstParamTy;
use std::mem;

pub use SectionKind::{Column, Row, Square};
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum SectionKind {
    Row,
    Column,
    Square,
}

/// A Section of the Grid, i.e. a Row, Column or Square.
/// # Comment:
/// This const param definition allows for generic and specific implementations;
/// as well as the more explicit nature of the Section type being part of the type
/// ```rust
/// impl<const K: SectionKind> Section<K> {/**/}
/// impl Section<{ Row }> {/**/}
/// ```
pub struct Section<const K: SectionKind> {
    idx: SectionIdx,
    pub cells: [Cell; 9],
}

impl<const K: SectionKind> Section<K> {
    pub fn new(grid: &Grid, section_idx: SectionIdx) -> Section<K> {
        let section_grididxs = Grid::section_grididxs(K, section_idx);
        let cells = grid.get_cells(section_grididxs);
        Section::<K> {
            idx: section_idx,
            cells,
        }
    }
}

impl Display for Section<{ Row }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {}",
            self.cells[0],
            self.cells[1],
            self.cells[2],
            self.cells[3],
            self.cells[4],
            self.cells[5],
            self.cells[6],
            self.cells[7],
            self.cells[8]
        )
    }
}

impl Display for Section<{ Column }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {}",
            self.cells[0],
            self.cells[1],
            self.cells[2],
            self.cells[3],
            self.cells[4],
            self.cells[5],
            self.cells[6],
            self.cells[7],
            self.cells[8]
        )
    }
}

impl Display for Section<{ Square }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^n$} {:^n$} {:^n$}\n{:^n$} {:^n$} {:^n$}\n{:^n$} {:^n$} {:^n$}",
            self.cells[0],
            self.cells[1],
            self.cells[2],
            self.cells[3],
            self.cells[4],
            self.cells[5],
            self.cells[6],
            self.cells[7],
            self.cells[8],
            n = 11
        )
    }
}

/// SIMD Stuff
mod section_simd {
    pub use super::*;
    use std::arch::x86_64::*;
    use std::simd::Simd; // requires `portable_simd`

    /// A wrapper for `__m256i` containing `section.cells` as `[u16; 9]` in its first 18 bytes.
    /// Zero filled in the remaining bytes.
    pub struct SimdSection(__m256i);

    impl<const K: SectionKind> From<Section<K>> for SimdSection {
        fn from(value: Section<K>) -> Self {
            // extend array to [u16; 16] via a zero fill on RHS
            let extended_section: [u16; 16] = {
                let mut zeros = [0; 16];
                zeros[..9].copy_from_slice(unsafe { &mem::transmute::<_, [u16; 9]>(value.cells) });
                zeros
            };

            SimdSection(Simd::from(extended_section).into())
        }
    }

    impl SimdSection {
        pub unsafe fn contains_mask(self, num: Num) -> u16 {
            let mask = 1 << i16::from(num);

            // load the mask into the SIMD register
            let mask_vector = _mm256_set1_epi16(mask);

            // perform bitwise AND
            let and_result = _mm256_and_si256(self.0, mask_vector);

            // compare the result against zero
            let zero_vector = _mm256_setzero_si256();
            let cmp_result = _mm256_cmpeq_epi16(and_result, zero_vector);
            !(_mm256_movemask_epi8(cmp_result) as u16)
        }
    }
}
pub use section_simd::*;
