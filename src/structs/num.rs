#![allow(unused)]

use crate::structs::*;
use std::mem::transmute;

/// The classical form of a known number in a `Cell`.
/// A number `1 <= N <= 9`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Num(u8);

#[derive(Debug)]
pub enum NumErr {
    Zero,
    TooBig,
}

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

macro_rules! impl_from_num {
    ($t: ty) => {
        impl From<Num> for $t {
            fn from(value: Num) -> Self {
                value.0 as $t
            }
        }
    };
}

impl_from_num!(u8);
impl_from_num!(u16);
impl_from_num!(i16);
