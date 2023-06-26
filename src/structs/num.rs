#![allow(unused)]

use crate::structs::*;
use std::mem::transmute;

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

impl Num {
    // Creates a Self from u8
    // Panics if !(1 <= n <= 9)
    pub fn new(n: u8) -> Self {
        assert!(0 < n && n < 10);
        unsafe { transmute(n) }
    }

    pub fn new_unchecked(n: u8) -> Self {
        unsafe { transmute(n) }
    }

    pub fn to_mask(self) -> Cell {
        unsafe { Cell::new_unchecked(!(1 << self as u16)) }
    }
}

impl TryFrom<u8> for Num {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 || value > 9 {
            return Err(());
        }
        Ok(Self::new_unchecked(value))
    }
}
