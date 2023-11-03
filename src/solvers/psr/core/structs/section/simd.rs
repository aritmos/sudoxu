use std::arch::x86_64::*;

use super::super::num::Num;

use super::Section;

/// A SIMD-compatible version of a [Section].
///
/// Obtained by packing a [`Section`]'s underlying bits into a [`__m256i`] zero-filled register.
pub struct SimdSection(__m256i);

impl From<Section> for SimdSection {
    fn from(value: Section) -> Self {
        let mut extended_array = [0; 16];
        extended_array[..9]
            .copy_from_slice(unsafe { &std::mem::transmute::<_, [u16; 9]>(value.cells) });

        // produces same output as going through `Simd::from`.
        let inner: __m256i =
            unsafe { _mm256_loadu_si256(extended_array.as_ptr() as *const __m256i) };
        SimdSection(inner)
    }
}

impl SimdSection {
    /// Checks inner [`Cells`](super::super::cell::Cell) for containing `n` as a candidate, and
    /// stores result in a mask.
    pub unsafe fn contains_mask(self, n: Num) -> u16 {
        let i16_mask = 1 << u16::from(n);

        let mask_splat = _mm256_set1_epi16(i16_mask);
        let zero_splat = _mm256_setzero_si256();

        // Perform bitwise `AND` and check against `0i16` on each lane.
        let and_result = _mm256_and_si256(self.0, mask_splat);
        // AVX512(BW + VL) Implementation:
        // `_mm256_cmpeq_epi16_mask(and_result, zero_splat)`
        // AVX2 Implementation:
        let cmp_result = _mm256_cmpeq_epi16(and_result, zero_splat);
        _mm256_movemask_epi8(cmp_result) as u16
    }
}
