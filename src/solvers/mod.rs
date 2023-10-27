//! Sudoku solver functionality.

mod simple;

use crate::structs::grid::Grid;

/// A sudoku solver.
pub trait Solver {
    /// Initialisation taking in a [`Grid`].
    fn init(grid: Grid) -> Self;

    /// Solve the grid. The returned [`Grid`] is expected to be solved.
    fn solve(self) -> Grid;
}
