use super::super::{cell::Cell, num::Num};
use super::{Grid, GridError};

use std::fmt::Display;

impl TryFrom<String> for Grid {
    type Error = GridError;

    /// Attempts to convert a `String` into a Grid.
    /// Ignores any non ascii digit characters.
    /// Fails if the number of numerical characters is not 81.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut grid = Grid::default();
        let nums = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        if nums.len() != 81 {
            return Err(GridError::FromStringError);
        }

        for (cell, num) in grid.0.iter_mut().zip(nums) {
            if num == 0 {
                continue;
            } else {
                *cell = Cell::new_known(unsafe { Num::new_unchecked(num as u8) });
            }
        }

        Ok(grid)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = unsafe {
            std::mem::transmute::<_, [[char; 9]; 9]>(self.0.map(char::from)).map(|r| {
                format!(
                    "│ {} {} {} │ {} {} {} │ {} {} {} │",
                    r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]
                )
            })
        };
        write!(
            f,
            "\
            ┌───────────────────────┐\n\
            {}\n\
            {}\n\
            {}\n\
            │ ──────┼───────┼────── │\n\
            {}\n\
            {}\n\
            {}\n\
            │ ──────┼───────┼────── │\n\
            {}\n\
            {}\n\
            {}\n\
            └───────────────────────┘\n\
            ",
            r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{SUDOKU_1_UNSOLVED_FMT_STR, SUDOKU_1_UNSOLVED_STR};

    #[test]
    fn grid_from_str() {
        let s = SUDOKU_1_UNSOLVED_STR.to_string();
        assert!(Grid::try_from(s).is_ok());
    }

    #[test]
    fn grid_to_str() {
        let grid = Grid::try_from(SUDOKU_1_UNSOLVED_STR.to_string()).unwrap();
        let grid_string = grid.to_string();
        let expected = SUDOKU_1_UNSOLVED_FMT_STR.to_string();
        assert_eq!(grid_string, expected)
    }
}
