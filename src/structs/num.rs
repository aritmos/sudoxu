#![allow(unused)]

use crate::structs::*;
use std::mem::transmute;

/// The classical form of a known number in a `Cell`.
/// A number `1 <= N <= 9`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Num {
    _1 = 1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

#[derive(Debug)]
pub enum NumErr {
    Zero,
    TooBig,
}

impl Num {
    pub fn new(n: u8) -> Result<Self, NumErr> {
        match n {
            0 => Err(NumErr::Zero),
            1..=9 => Ok(unsafe { transmute(n) }),
            10.. => Err(NumErr::TooBig),
        }
    }

    /// # Safety
    /// The caller must ensure that the `u8` passed in satisfies `1 <= n <= 9`.
    pub unsafe fn new_unchecked(n: u8) -> Self {
        unsafe { transmute(n) }
    }
}
