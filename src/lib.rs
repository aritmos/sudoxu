#![warn(missing_docs)]
#![allow(unused)]
#![allow(clippy::unusual_byte_groupings)]

//! # Sudoxu
//! Sudoxu is a library

pub mod structs;
#[doc(inline)]
use structs::*;

pub mod finders;

pub mod solvers;

mod tests;
// mod utils;
