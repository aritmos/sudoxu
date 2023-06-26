use crate::structs::*;
use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A u16 with the following bite representation:
/// (000 000) 0 0 0  0 0 0  0 0 0  0
/// .         ^^^^^^^^^^^^^^^^^^^  ^
/// .         candidates           known
/// .         9 8 7  6 5 4  3 2 1
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell(u16);

pub enum CellError {
    ProhibittedBits,        // Cell >= 1024 (= 2^10)
    KnownNoNum,             // Cell == 1
    KnownMultipleNum,       // Cell % 2 == 1 && !(Cell - 1).is_power_of_two()
    NoCandidates,           // Cell == 0
    MultipleSoleCandidates, // Multiple single candidates within sections
}

/// Option<Num> to signify zero or one sole candidates
/// Err if multiple or no possible candidates
pub type CellResult = Result<Option<Num>, CellError>;

/// Base implementations
impl Cell {
    pub unsafe fn new_unchecked(n: u16) -> Self {
        Self(n)
    }

    // TODO: create proper error variant
    pub fn new(n: u16) -> Result<Self, CellError> {
        // u16 cant be too small or too large
        match n {
            0 => return Err(CellError::NoCandidates),
            1024.. => return Err(CellError::ProhibittedBits),
            _ => (),
        }

        // unknown cell
        if n % 2 != 1 {
            return Ok(Self(n));
        }

        // known cell
        match n.count_ones() {
            0 => unreachable!(),
            1 => Err(CellError::KnownNoNum),
            2 => Ok(Self(n)),
            3.. => Err(CellError::KnownMultipleNum),
        }
    }

    pub fn from_candidates(candidates: &[Num]) -> Self {
        let mut cell_u16: u16 = 0;
        for num in candidates {
            cell_u16 |= 1 << *num as u8;
        }
        Self(cell_u16)
    }

    pub fn get_mut(&mut self) -> &mut u16 {
        &mut self.0
    }

    /// COMMENT: this gets optimised out to `self.0` which
    /// ends up being the same as directly working with a `u16`
    pub fn to_u16(self) -> u16 {
        self.0
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self(0b0000001111111110)
        //           987654321
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prohibitted_bits = self.0 >> 10;
        let main_bits = self.0 & 0b1111111111;
        write!(f, "({:0>6b}){:0>10b}", prohibitted_bits, main_bits)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell_u16 = self.to_u16();
        let cell_str = String::new();
        if cell_u16 % 2 == 1 {
            let n = (cell_u16 & !1).ilog2();
            write!(f, "{}", n.to_string())
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

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        if value.is_known() {
            let n = value.0.ilog2();
            char::try_from(n).unwrap()
        } else {
            ' '
        }
    }
}

/// Finds if any Cell in the slice can hold a certain Num
pub fn contains(cells: &[Cell], n: Num) -> bool {
    let combined_u16 = cells.iter().map(|&c| c.to_u16()).fold(0, |a, b| a | b);
    combined_u16 & (1 << n as u8) != 0
}

impl BitAnd for Cell {
    type Output = Cell;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Cell::new_unchecked(self.to_u16() & rhs.to_u16()) }
    }
}

impl BitAndAssign for Cell {
    fn bitand_assign(&mut self, rhs: Self) {
        *self.get_mut() &= rhs.to_u16();
    }
}

impl BitOr for Cell {
    type Output = Cell;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Cell::new_unchecked(self.to_u16() | rhs.to_u16()) }
    }
}

impl BitOrAssign for Cell {
    fn bitor_assign(&mut self, rhs: Self) {
        *self.get_mut() |= rhs.to_u16();
    }
}

impl BitXor for Cell {
    type Output = Cell;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Cell::new_unchecked(self.to_u16() ^ rhs.to_u16()) }
    }
}

impl BitXorAssign for Cell {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self.get_mut() ^= rhs.to_u16();
    }
}

impl Not for Cell {
    type Output = Cell;

    fn not(self) -> Self::Output {
        let not_bits = unsafe { Self::new_unchecked(!self.to_u16()) };
        not_bits & Self::default()
    }
}

/// Functionality implementations
impl Cell {
    pub fn is_known(&self) -> bool {
        self.to_u16() % 2 == 1
    }

    // removes Num from the candidates in Self
    pub fn remove_candidate(&mut self, num: Num) {
        self.mask(num.to_mask());
    }

    pub fn mask(&mut self, mask: Cell) {
        *self &= mask;
    }

    pub fn contains_candidate(self, num: Num) -> bool {
        self.to_u16() & (1 << num as u8) != 0
    }

    /// Returns a `Cell` whose candidates are the known numbers within the square
    pub fn combine_candidates(cells: &[Cell]) -> Cell {
        let mut known = unsafe { Cell::new_unchecked(0) };
        for cell in cells {
            if cell.is_known() {
                known |= *cell;
            }
        }
        known &= Cell::default(); // mask the "known" bit to 0
        known
    }
}
