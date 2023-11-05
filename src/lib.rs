#![warn(missing_docs)]
#![allow(unused)]
#![allow(clippy::unusual_byte_groupings)]
// used to turn [T; N] into SIMD types such as `__m256`.
// currently using loads and pointer casts to avoid use.
// #![feature(portable_simd)]
// Direct access to certain SIMD functions
#![feature(stdsimd)]

//! # Sudoxu
//! A Rust library for sudoku solving.
//!
//! For parity with the codebase, always clone the repo and locally compile the documentation using
//! `rustup` or `cargo doc`.
//!
//! ## Project Structure

pub mod board;
pub mod solver;
pub mod solvers;
