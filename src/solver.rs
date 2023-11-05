//! The root `Solver` trait, implemented for all types which can solve sudokus.

use crate::board::{Board, BoardError};

/// A sudoku solver.
pub trait Solver: Sized {
    /// Initialize `self` from a [Board].
    fn init(grid_string: Board) -> Self;

    /// Compute the solution and return the solved board.
    fn solve(self) -> Board;
}

/// Parses the `&str` into a [`Board`] and solves it with the specified [`Solver`].
pub fn solve_board<S: Solver>(board_str: &str) -> Result<Board, BoardError> {
    let board = Board::try_from(board_str)?;
    let solver = S::init(board);
    Ok(solver.solve())
}

/// A sudoku solver that shows its steps into solving the sudoku.
pub trait StepSolver: Solver {
    /// Computes the following step towards solving the sudoku.
    /// Returns the state of the [Board] after computing the next step along with an optional
    /// `String` logging any details of the computations.
    /// Once the sudoku board has been solved the `Solver` will return `None`
    fn step(&mut self) -> Option<Board>;
}

struct StepSolverIterator<S: StepSolver>(S);

impl<S: StepSolver> Iterator for StepSolverIterator<S> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// A solver which logs its solution to the sudoku.
pub trait LoggingSolver: Solver {
    /// Solves the board while producing logs of each step.
    fn solve_with_logs(self) -> (Board, Vec<String>);
}

/// A solver that steps through its process of solving the board.
pub trait StepLoggingSolver: Solver {
    /// Solves the board one step a time.
    fn step(&mut self) -> Option<(Board, String)>;
}
