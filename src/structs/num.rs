/// A known number within a `Cell`, satisfying `1 <= N <= 9`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Num(u8);

#[derive(Debug)]
pub enum NumErr {
    Zero,   // n == 0
    TooBig, // n > 9
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
    pub fn new(n: u8) -> Result<Self, NumErr> {
        match n {
            0 => Err(NumErr::Zero),
            1..=9 => Ok(Self(n)),
            10.. => Err(NumErr::TooBig),
        }
    }

    /// # Safety
    /// The caller must ensure that the `u8` passed in satisfies `1 <= n <= 9`.
    pub unsafe fn new_unchecked(n: u8) -> Self {
        Self(n)
    }
}
