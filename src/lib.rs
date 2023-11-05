#![warn(missing_docs)]
#![allow(unused)]
#![allow(clippy::unusual_byte_groupings)]
// used to turn [T; N] into SIMD types such as `__m256`.
// currently using loads and pointer casts to avoid use.
// #![feature(portable_simd)]
// Direct access to certain SIMD functions
#![feature(stdsimd)]

//! # Sudoxu
//! A Rust library (and eventually also a binary) for sudoku solving.
//!
//! #### `!Sync` Documentation.
//! The version of this project's documentation hosted in GitHub pages may not always align with
//! the state of the [main repo](https://github.com/aritmos/sudoxu).
//! For parity with the codebase, compile the documentation from source (requires Rust toolchain)
//! ```txt
//! > git clone --depth 1 https://github.com/aritmos/sudoxu.git
//! > cd sudoxu
//! > cargo doc
//! > $BROWSER target/docs/sudoxu/index.html
//! ```
//!
//! # Quick Start
//! The project is still early in production and not fully functional. Defined solvers may be
//! tested using the following code:
//! ```
//! use sudoku::prelude::*;
//!
//! // Create a board using an `&str` within the library's database (Easy #01 unsolved).
//! let board = Board::try_from(boards::E01_U).unwrap();
//!
//! // Solve the board using the [`btr::Backtracker`] solver and print the result.
//! solve_board<btr::Backtracker>(board).print();
//! ```
//!
//! # Introduction
//!
//! The current state of the crate defines the [`Board`](board::Board) struct and
//! [`Solver`](solver::Solver) trait. The [`Board`](board::Board) acts the interface between I/O
//! and the Solvers. Solvers are types which implement the [`Solver`](solver::Solver) trait.
//!
//! ```text
//!   I/O                 sudoku          
//! ┌──────┐     ┌───────────────────────┐
//! │ &str │ <-> │ Board <-> impl Solver │
//! └──────┘     └───────────────────────┘
//! ```
//!
//! Solvers have two key distinctions between one another:
//!  1. The fundamental way in which they solve a sudoku --- i.e. the techniques.
//!  2. How they solve the sudoku --- i.e. the strategy.
//!
//! In some cases a given group of sudoku techniques might only have a single strategy. However in
//! more complex cases (such as how humans solve sudokus) there can be multiple strategies.
//!
//! A technique or group of techniques define a `Solver` "category".
//! Solver categories are the submodules of the [`solvers`] module.
//! Each category can have multiple inner Solvers, which can utilize different strategies to solve
//! the sudoku using the same techniques. In these cases, the common techniques are stored within a
//! `core` module, acting as a library for all of the solvers of that category.
//!
//! ## Solver Categories
//! #### Backtracking ([`btr`](solvers::btr))
//! The standard backtracking solution. This category contains a
//! single solver.
//!
//! #### Phase Space Reduction ([`psr`](solvers::psr))
//!
//! The inception of this crate.
//!
//! A computerized version of a human's approach to sudoku solving. Removes candidates from
//! the board using traditional techniques such as single candidates, unique candidates or
//! candidate projections. Unlike humans however, it begins by filling the entire board with
//! all pencilmarks {1,...,9} (except the starting cells), and removing the pencilmarks step-by-step.
//! These solvers therefore reduce the phase space of allowed boards until only a signle allowed
//! state is left, the solution.
//!
//! There are many strategies for solving sudokus using traditional techniques, ranging from an
//! amateur's approach to an expert's. Implementations of [`psr`](solvers::psr) solvers use a range
//! of techniques; normally an increasing set ordered by technique difficulty or computational
//! complexity.
//!
//! Current Capabilities/Techniques
//! (See the [module level documentation](solvers::psr) for more information):
//!
//! * Easy:
//!    - [x] Single Candidate
//!    - [x] Unique Candidate
//! * Medium:
//!    - [x] Candidate Lines
//!    - [x] Double Pairs
//!    - [x] Multiple Lines
//! * Advanced:
//!    - [ ] Naked Pairs/Triples
//!    - [ ] Hidden Pairs/Triples
//! * Master:
//!    - [ ] X-Wing
//!    - [ ] Swordfish
//!    - [ ] Forcing Chains
//!    - [ ] Nishio
//!
//! These solvers were created with the eventual goal of being able to guide a human into solving a
//! sudoku puzzle. Possibly by taking in a partially completed board and outlining the steps
//! towards solving the puzzle.
//!
//! # Project Goals
//!
//! This project is an primarily an exploration, and secondly an actual crate. It originally began
//! as the implementation for the core functionality of the [`psr`](solvers::psr) solvers,
//! aimed at exploring how to efficiently translate human techniques into a computational
//! algorithm.
//!
//! The [core psr functionality](solvers::psr::core) was invisioned to be efficient and performant.
//! The implementations focus on a lack of heap allocation (although some minimal
//! allocation has been incorporated to make parts of the code cleaner), strong emphasis on bit
//! manipulation, SIMD, and vendor intrinsics. The latter of these are still barely a part of the
//! current implementations, however they play stronger roles in some of the currently unimplemented
//! (but already fully devised) Advanced+ techniques.
//!
//! Now the project has expanded beyond the [`psr`](solvers::psr) to allow for multiple solver
//! categories, and is focused on eventually becoming a full blown TUI application for sudoku
//! solving.
//!
//! ## Project Roadmap
//!
//! The essential implementation ordering is as outlined below, although I commonly work on what I
//! feel like doing. Work towards a major version can occur before a prior major version is
//! finished.
//!
//! * `v0.2.0` (Core lib)
//!     * Expand the core library with a wide range of solving techniques and multiple solver
//!     categories.
//!     * Document modules and implementations
//! * `v0.3.0` (Fully functional lib)
//!     * Ability to run multiple solvers using a range of strategies
//!     * Automatic testing of solvers and performance evaluation
//!     * Multiple [`Solver`](solver::Solver) types, including solvers that log their solution,
//!     show steps, or outline their limitations.
//! * `v0.4.0` (TUI Binary)
//!     * Implement a terminal UI interface, with the ability to load sudokus from a multiple
//!     sources
//!     * Show (certain) solvers solving the sudoku in real time

/*
┌───────────────────────┐
│ ──────┼───────┼────── │
└───────────────────────┘
*/

pub mod board;
pub mod solver;
pub mod solvers;

/// Prelude module, used for easy access to solvers and functions.
#[macro_export]
pub mod prelude {
    pub use super::solver::solve_board;
    pub use super::solvers::utils::{boards::*, time::time};
    pub use super::solvers::*;
}
