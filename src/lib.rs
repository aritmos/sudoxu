#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(clippy::missing_safety_doc)]
#![feature(stdsimd)]
#![feature(slice_flatten)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_write_slice)]

mod structs;
pub use structs::*;

mod utils;
pub use utils::*;

mod filters;
pub use filters::*;

mod finders;
pub use finders::*;

mod test;
pub use test::*;
