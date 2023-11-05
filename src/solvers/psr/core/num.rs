/// A known number within a `Cell`, satisfying `1 <= N <= 9`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Num(u8);

#[derive(Debug)]
/// [`Num`]-related errors.
pub enum NumErr {
    /// `n == 0`
    Zero,
    /// `n > 9`
    TooBig,
}

macro_rules! impl_from_num {
    ($t: ty) => {
        impl From<Num> for $t {
            fn from(n: Num) -> Self {
                n.0 as $t
            }
        }
    };
}

impl_from_num!(u16);

impl Num {
    /// Creates a [`Num`] from a `u8`.
    /// Returns an error if `n` is not in `1..=9`.
    pub fn new(n: u8) -> Result<Self, NumErr> {
        match n {
            0 => Err(NumErr::Zero),
            1..=9 => Ok(Self(n)),
            10.. => Err(NumErr::TooBig),
        }
    }

    /// Creates a [`Num`] from a `u8`. Does not apply the bounds check.
    /// # Safety
    /// The caller must ensure that `1 <= n <= 9`.
    pub unsafe fn new_unchecked(n: u8) -> Self {
        Self(n)
    }
}
