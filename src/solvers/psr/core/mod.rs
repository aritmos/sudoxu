/// The cells in a sudoku grid.
pub mod cell;

/// The sudoku board.
pub mod grid;

/// Safe indexing into comptime known arrays.
pub mod idx;

/// Representation of a known number within a [`Cell`](cell::Cell): `1 <= N <= 9`.
pub mod num;

/// A row, column, or box within the grid.
pub mod section;

/// Filter type, used to remove candidates from [`Cells`](cell::Cell).
pub mod filter;

/// Adjacent [`Sections`](section::Section) forming a third of the board.
pub mod area;
