//! I/O representation of sudoku boards.

use std::fmt::{Debug, Display};

/// I/O version of the sudoku board.
/// Each byte is guaranteed to be an ascii digit.
pub struct Board(pub [u8; 81]);

/// [`Board`] related errors.
pub enum BoardError {
    /// Error parsing an `&str` into a [`Board`].
    FromStrError,
}

impl TryFrom<&str> for Board {
    type Error = BoardError;

    /// Parses a `String` into a [Board].
    /// The parsing ignores any `char`s that are not (base 10) digits.
    /// Returns an error if the number of digits is not equal to 81.
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|n| n as u8))
            .collect::<Vec<u8>>()
            .try_into()
        {
            Ok(a) => Ok(Self(a)),
            Err(_) => Err(BoardError::FromStrError),
        }
    }
}

impl Board {
    /// Prints `self` using the [`Display`] trait.
    pub fn print(&self) {
        println!("{self}");
    }

    /// Prints `self` using the [`Debug`] trait.
    pub fn dbg(&self) {
        println!("{self:?}");
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.chunks(9) {
            let spaced_row = {
                let mut tmp = [b' '; 17];
                for i in 0..9 {
                    tmp[2 * i] = row[i] + b'0';
                }
                tmp
            };
            writeln!(f, "{}", unsafe {
                std::str::from_utf8_unchecked(&spaced_row)
            })?;
        }
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = unsafe { std::mem::transmute::<_, [[u8; 9]; 9]>(self.0) };
        let mut fmt_row_iter = rows.into_iter().map(|r| {
            format!(
                "│ {} {} {} │ {} {} {} │ {} {} {} │",
                r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8],
            )
        });
        writeln!(f, "┌───────────────────────┐")?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "│ ──────┼───────┼────── │")?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "│ ──────┼───────┼────── │")?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        writeln!(f, "{}", fmt_row_iter.next().unwrap())?;
        write!(f, "└───────────────────────┘")?;
        Ok(())
    }
}
