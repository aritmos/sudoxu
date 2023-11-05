use super::{Cell, Grid, Num};
use crate::board::Board;

impl From<Board> for Grid {
    fn from(board: Board) -> Self {
        Self(board.0.map(|b| match b - b'0' {
            0 => Cell::default(),
            n => Cell::new_known(unsafe { Num::new_unchecked(n) }),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board() {
        let s = std::string::String::from_utf8(vec![b'0'; 81]).unwrap();
        let b = Board::try_from(s.as_str()).unwrap();
        assert_eq!(Grid::from(b), Grid::default());
    }
}
