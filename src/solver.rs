//! The root `Solver` trait, implemented for all types which can solve sudokus.

use crate::board::Board;

/// A sudoku solver.
pub trait Solver: Sized {
    /// Initialize self
    fn init(grid_string: Board) -> Self;

    /// Solve the sudoku and return the solved board.
    fn solve(self) -> Board;
}
