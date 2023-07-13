/// `Idx<N>` represents an integer `x` guaranteed to satisfy: `0 <= x < N`
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

impl<const N: u8> TryFrom<u8> for Idx<N> {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < N {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl<const N: u8> From<Idx<N>> for u8 {
    fn from(value: Idx<N>) -> Self {
        value.0
    }
}

impl<const N: u8> From<Idx<N>> for usize {
    fn from(value: Idx<N>) -> Self {
        value.0 as usize
    }
}
