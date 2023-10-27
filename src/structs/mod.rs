//! Structs representing various parts of a sudoku grid, along with helper types.

/// The cells in a sudoku grid.
pub mod cell;

/// the sudoku grid
pub mod grid;

/// safe indexing into comptime known arrays
pub mod idx;

/// the known value within a cell. `1 <= N <= 9`.
pub mod num;

/// A row, column, or box within the grid.
/// Internally a `[Cell; 9]`
pub mod section;
