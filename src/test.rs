#![allow(unused_imports)]
use crate::{structs::*, utils::*};
use std::hint::black_box;
use std::time::Instant;

pub fn test() {
    let t_i = Instant::now();
    for _ in 0..10_000_usize {
        black_box(todo!())
    }
    let dt = Instant::now().duration_since(t_i);
    println!("{dt:?}");
}
