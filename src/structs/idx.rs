use crate::structs::*;

#[derive(Clone, Copy)]
pub struct Idx<const N: u8>(u8);

#[derive(Debug)]
pub enum IdxErr {
    TooBig,
}

impl<const N: u8> Idx<N> {
    /// Create an Idx<N> from a `u8`
    pub fn new(n: u8) -> Result<Self, IdxErr> {
        if n < N {
            Ok(Self(n))
        } else {
            Err(IdxErr::TooBig)
        }
    }

    /// # Safety
    /// The caller must ensure that the `u8` passed in is less than `N`
    pub unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}

// For index value computation
impl<const N: u8> From<Idx<N>> for u8 {
    fn from(value: Idx<N>) -> Self {
        value.0
    }
}

// For indexing into slices without the SliceIndex trait for now
impl<const N: u8> From<Idx<N>> for usize {
    fn from(value: Idx<N>) -> Self {
        value.0 as usize
    }
}

impl<T, const N: u8> std::ops::Index<Idx<N>> for [T; N as usize] {
    type Output = T;

    fn index(&self, index: Idx<N>) -> &Self::Output {
        unsafe { self.get_unchecked(index.0 as usize) }
    }
}

impl<T, const N: u8> std::ops::IndexMut<Idx<N>> for [T; N as usize] {
    fn index_mut(&mut self, index: Idx<N>) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index.0 as usize) }
    }
}

// let idx: u8 = idx.into();
// let row_idx = idx / 9;
// let col_idx = idx % 9;
// let square_idx = 3 * (row_idx / 3) + (col_idx / 3);
// let inner_square_idx = { 3 * (row_idx % 3) + (col_idx % 3) };

impl GridIdx {
    #[inline(always)]
    pub fn row_idx(self) -> SectionIdx {
        let row_idx_u8 = u8::from(self) / 9;
        unsafe { SectionIdx::new_unchecked(row_idx_u8) }
    }

    #[inline(always)]
    pub fn inner_row_idx(self) -> InnerIdx {
        self.col_idx()
    }

    #[inline(always)]
    pub fn col_idx(self) -> SectionIdx {
        let col_idx_u8 = u8::from(self) % 9;
        unsafe { SectionIdx::new_unchecked(col_idx_u8) }
    }

    #[inline(always)]
    pub fn inner_col_idx(self) -> InnerIdx {
        self.row_idx()
    }

    // pub fn square_idx(self) -> SectionIdx;
    // pub fn inner_square_idx(self) -> InnerIdx;
}
