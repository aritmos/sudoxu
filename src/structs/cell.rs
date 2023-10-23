use crate::structs::*;
use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use std::ops::{Deref, DerefMut};

/// A u16 with the following bit representation:
/// (000 000) 0 0 0  0 0 0  0 0 0  0
/// .         ^^^^^^^^^^^^^^^^^^^  ^
/// .         candidates           known
/// .         9 8 7  6 5 4  3 2 1
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell(u16);

pub enum CellError {
    ParseError,
    ProhibittedBits,        // Cell >= 1024 (= 2^10)
    KnownNoNum,             // Cell == 1
    KnownMultipleNum,       // Cell % 2 == 1 && !(Cell - 1).is_power_of_two()
    NoCandidates,           // Cell == 0
    MultipleSoleCandidates, // Multiple single candidates within sections
}

/// Option<Num> to signify zero or one sole candidates
/// Err if multiple or no possible candidates
pub type CellResult = Result<Option<Num>, CellError>;

/// Allows us to use `u16` methods on `Cell`
impl Deref for Cell {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Base implementations
impl Cell {
    /// # Safety
    /// The caller must ensure that the content of the `u16` is a valid representation of a `Cell`.
    pub unsafe fn new_unchecked(n: u16) -> Self {
        Self(n)
    }

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

    /// Returns a `Cell` representing a known `n`.
    pub fn new_known(n: Num) -> Cell {
        let inner: u16 = (1 << u16::from(n)) | 1;
        unsafe { Cell::new_unchecked(inner) }
    }

    pub fn from_candidates(candidates: &[Num]) -> Self {
        let mut cell_u16: u16 = 0;
        for num in candidates {
            cell_u16 |= 1 << u8::from(*num);
        }
        Self(cell_u16)
    }

    /// Equivalent functionality to dereferncing.
    /// Used to better showcase intent within the code.
    /// This operation is zero cost as far as I can see.
    #[inline]
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
        if cell_u16 % 2 == 1 {
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

impl TryFrom<&str> for Cell {
    type Error = CellError;

    /// Creates a `Cell` from a `String`
    /// # Examples (in the `u16` representation)
    /// ```rust
    ///    // `0` is treated as all possible candidates
    ///    Cell::try_from("0") == `Ok(Cell(0b000000_111_111_111_0))`
    ///    // every other single digit is treated as a known cell
    ///    Cell::try_from("7") == `Ok(Cell(0b000000_001_000_000_1))`
    ///    // candidates are wrapped by `{}`
    ///    Cell::try_from("{123}") = Ok(Cell(0b000000_000_000_111_0))
    ///    // all else will fail to convert
    ///    Cell::try_from("35") = Err(CellError::ParseError)
    ///    Cell::try_from("10") = Err(CellError::ParseError)
    ///    Cell::try_from("{16") = Err(CellError::ParseError)
    ///    Cell::try_from("{45]") = Err(CellError::ParseError)
    ///    Cell::try_from("{a}") = Err(CellError::ParseError)
    ///    Cell::try_from("{a8}") = Err(CellError::ParseError)
    ///    Cell::try_from("f") = Err(CellError::ParseError)
    /// ```
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.len() {
            // known cell or blank
            1 => {
                let Ok(n) = s.parse::<u8>() else {
                    return Err(CellError::ParseError);
                };
                match n {
                    0 => Ok(Cell::default()), // treat `0` as all possible candidates
                    n => Ok(Cell::new_known(unsafe { Num::new_unchecked(n) })),
                }
            }
            // candidates
            3.. => {
                let s = s.as_bytes();
                // check the wrapping chars
                if (s.first().unwrap() != &b'{') || (s.last().unwrap() != &b'}') {
                    return Err(CellError::ParseError);
                }
                let nums = &s[1..s.len() - 1];
                let mut inner = 0u16;
                for num in nums {
                    if (49..=57).contains(num) {
                        // check that b'1' <= num <= b'9'
                        let n = num - 48;
                        inner |= 1 << n;
                    } else {
                        return Err(CellError::ParseError);
                    }
                }
                Ok(unsafe { Cell::new_unchecked(inner) })
            }
            _ => Err(CellError::ParseError),
        }
    }
}

/// Is this used anywhere?
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

impl BitAnd for Cell {
    type Output = Cell;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Cell::new_unchecked(*self & *rhs) }
    }
}

impl BitAndAssign for Cell {
    fn bitand_assign(&mut self, rhs: Self) {
        **self = *rhs;
    }
}

impl BitOr for Cell {
    type Output = Cell;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Cell::new_unchecked(*self | *rhs) }
    }
}

impl BitOrAssign for Cell {
    fn bitor_assign(&mut self, rhs: Self) {
        **self |= *rhs;
    }
}

impl BitXor for Cell {
    type Output = Cell;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Cell::new_unchecked(*self ^ *rhs) }
    }
}

impl BitXorAssign for Cell {
    fn bitxor_assign(&mut self, rhs: Self) {
        **self ^= *rhs;
    }
}

impl Not for Cell {
    type Output = Cell;

    /// Flips only the candidate bits in a `Cell`
    ///
    /// # Safety
    /// Method is technically `unsafe` as it can produce a `CellError::NoCandidates`
    /// output. However `!Cell` is only meant to be used within bitmask operations,
    /// hence it should be verified by the caller within its usecases that the actual
    /// output of the entire calculation produces a valid `Cell`.
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!(!Cell::new(0b111_000_000_0), Cell::new(0b000_111_111_0))
    /// assert_eq!(!Cell::new(0b101_010_101_0), Cell::new(0b010_101_010_0))
    /// // Technically invalid `Cell` state output:
    /// assert_eq!(!Cell::new(0b111_111_111_0), Cell::new(0b000_000_000_0))
    /// ```
    fn not(self) -> Self::Output {
        self ^ Self::default()
    }
}

/// Functionality implementations
impl Cell {
    #[inline(always)]
    pub fn is_known(self) -> bool {
        self.0 % 2 == 1
    }

    pub fn contains_candidate(self, num: Num) -> bool {
        (self & num.to_mask()).to_u16() != 0
    }

    /// Returns a "`Cell`" whose candidates are the known numbers within the provided slice
    pub fn combine_known(cells: &[Cell]) -> Cell {
        let mut known = unsafe { Cell::zero() };
        for cell in cells {
            if cell.is_known() {
                known |= *cell;
            }
        }
        known &= Cell::default(); // mask the "known" bit to 0
        known
    }

    /// Wrapper for `Cell::new_unchecked(0)`
    /// # Safety
    /// This is an INVALID `Cell` REPRESENTATION meant to be ONLY used within calculations/accumulations.
    /// Caller guarantees that this form of a `Cell` is only used witihn such cases
    /// and is never returned or used as an actual `Cell`.
    #[inline(always)]
    pub unsafe fn zero() -> Cell {
        Cell::new_unchecked(0)
    }

    /// Turns a known `Cell` into a `Num`.
    /// # Safety
    /// Function doesn't check if the `Cell` is actually known
    /// (only contains a single candidate),
    /// and simply takes the largest candidate.
    /// The caller ensures that the `Cell` is known.
    pub unsafe fn known_to_num(self) -> Num {
        let n = self.ilog2() as u8;
        unsafe { Num::new_unchecked(n) }
    }
}
