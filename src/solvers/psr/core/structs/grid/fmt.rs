use super::super::{cell::Cell, grid::Grid, num::Num};

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = unsafe { std::mem::transmute::<_, [[Cell; 9]; 9]>(self.0) };
        for r in rows {
            writeln!(
                f,
                "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]
            )?;
        }
        Ok(())
    }
}

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
        let b = Board::try_from(s).unwrap();
        assert_eq!(Grid::from(b), Grid::default());
    }
}
