#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(clippy::missing_safety_doc)]
#![feature(stdsimd)]
#![feature(slice_flatten)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

mod filters;
mod finders;
mod structs;
pub use structs::*;
mod utils;

fn main() {
    let mut cell = Cell::default();
    cell.remove_candidate(Num::new(4).unwrap());
    cell.remove_candidate(Num::new(5).unwrap());

    println!("{}", !cell);

    let _x = Idx::<9>::new(5_u8);
}
