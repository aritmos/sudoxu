use std::fmt::{Debug, Display};

use super::Cell;

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prohibitted_bits = self.to_u16() >> 10;
        let main_bits = self.to_u16() & 0b1111111111;
        write!(f, "({:0>6b}){:0>10b}", prohibitted_bits, main_bits)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell_u16 = self.to_u16();
        if self.is_known() {
            write!(f, "{}", cell_u16.ilog2())
        } else {
            let mut cell_str = String::new();
            for i in 1..=9 {
                if cell_u16 & (1 << i) != 0 {
                    cell_str += &i.to_string();
                }
            }
            write!(f, "{{{cell_str}}}")
        }
    }
}

// Used for printing the `Grid`.
impl From<Cell> for char {
    /// Known `Cell`s get mapped to their value.
    /// Unknown `Cell`s get mapped to ' '.
    fn from(cell: Cell) -> Self {
        if cell.is_known() {
            let n = cell.to_u16().ilog2();
            char::from_digit(n, 10).unwrap()
        } else {
            ' '
        }
    }
}

// Used for printing the `Grid`.
impl From<Cell> for u8 {
    /// Known `Cell`s get mapped to their value.
    /// Unknown `Cell`s get mapped to b' '.
    fn from(cell: Cell) -> Self {
        if cell.is_known() {
            cell.to_u16().ilog2() as Self + b'0'
        } else {
            b' '
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_char() {
        let cell = Cell::new(0b100_000_000_1).unwrap();
        assert_eq!(char::from(cell), '9');
    }

    #[test]
    fn byte_conversion() {
        let c = Cell::new(0b100_000_000_1).unwrap();
        let b = u8::from(c);
        assert_eq!(b, b'9');
    }
}
