use crate::structs::*;
use std::mem::transmute;

#[derive(Clone, Copy)]
pub struct Section([Cell; 9]);

pub type Square = Section;

pub enum SectionType {
    Row,
    Column,
    Square,
}

/// Wrapper for `[Cell; 9]`, representing rows, columns, and squares.
impl Section {
    pub fn new(cells: [Cell; 9]) -> Self {
        unsafe { transmute(cells) }
    }

    pub fn to_cells(self) -> [Cell; 9] {
        unsafe { transmute(self) }
    }

    pub fn to_string(self, section_type: SectionType) -> String {
        match section_type {
            SectionType::Row | SectionType::Column => format!(
                "{} {} {} {} {} {} {} {} {}",
                self.0[0],
                self.0[1],
                self.0[2],
                self.0[3],
                self.0[4],
                self.0[5],
                self.0[6],
                self.0[7],
                self.0[8]
            ),
            SectionType::Square => format!(
                "{:^n$} {:^n$} {:^n$}\n{:^n$} {:^n$} {:^n$}\n{:^n$} {:^n$} {:^n$}",
                self.0[0].to_string(),
                self.0[1].to_string(),
                self.0[2].to_string(),
                self.0[3].to_string(),
                self.0[4].to_string(),
                self.0[5].to_string(),
                self.0[6].to_string(),
                self.0[7].to_string(),
                self.0[8].to_string(),
                n = 11
            ),
        }
    }
}

/// SIMD Stuff
mod simd {
    pub use super::*;
    use std::arch::x86_64::*;

    impl From<Section> for __m256i {
        #[inline]
        fn from(value: Section) -> Self {
            let values: &[u16] = unsafe { std::mem::transmute(value.0.as_slice()) };

            // extend array to [u16; 16] via a zero fill on RHS
            let mut extended_section: [u16; 16] = [0; 16];
            extended_section[..9].copy_from_slice(values);

            let value_vector = unsafe { _mm256_loadu_si256(extended_section.as_ptr() as *const _) };
            // value_vector

            // create a mask "[u16; 16]" with values in {0, u16::MAX} based on the value of the LSB in
            // the original "[u16; 16]"
            let mask = unsafe { _mm256_cmpgt_epi16(value_vector, _mm256_setzero_si256()) };

            // result
            unsafe { _mm256_andnot_si256(mask, value_vector) }
        }
    }

    impl Section {
        pub unsafe fn contains_mask(values: __m256i, n: Num) -> u16 {
            let mask = 1 << n as i16;

            // load the mask into the SIMD register
            let mask_vector = _mm256_set1_epi16(mask);

            // perform bitwise AND
            let and_result = _mm256_and_si256(values, mask_vector);

            // compare the result against zero
            let zero_vector = _mm256_setzero_si256();
            let cmp_result = _mm256_cmpeq_epi16(and_result, zero_vector);
            !(_mm256_movemask_epi8(cmp_result) as u16)
        }
    }
}
pub use simd::*;
