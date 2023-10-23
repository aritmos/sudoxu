#![allow(unused_imports)]
use crate::{structs::*, utils::*};
use std::arch::x86_64::*;
use std::hint::black_box;
use std::simd::*;
use std::time::Instant;

pub fn bench() {
    let t_i = Instant::now();
    for _ in 0..10_000_usize {
        todo!() // black_box this
    }
    let dt = Instant::now().duration_since(t_i);
    println!("{dt:?}");
}

pub fn test() {}
