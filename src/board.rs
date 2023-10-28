//! I/O representation of sudoku boards.

/// I/O version of the sudoku board.
/// Each byte is guaranteed to be an ascii digit.
pub struct Board(pub [u8; 81]);
