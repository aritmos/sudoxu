#![allow(unused)]
#![allow(incomplete_features)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(generic_const_exprs)]
#![feature(slice_flatten)]

mod filters;

mod finders;

mod structs;
use structs::*;

fn main() {
    let mut cell = Cell::default();
    cell.remove_candidate(Num::new(4));
    cell.remove_candidate(Num::new(5));

    let mut x = cell.to_u16(); // copies;
    x = 3;
    println!("{cell}");

    let mut y = cell.get_mut();
    *y = 3;
    println!("{cell}");
}
