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
//! #### ⚠️ Ultra-Nightly ⚠️
//! This documentation is largely for my own use, and is frequently compiled during coding sessions.
//! Sometimes (although rarely), pushes are made to this documentation's repo ahead of the [main
//! repo](https://github.com/aritmos/sudoxu).
//!
//! For parity with the codebase, always clone the repo and locally compile the documentation using
//! `rustup` or `cargo doc`.
//!
//! ## Project Structure

pub mod board;
pub mod solver;
pub mod solvers;
