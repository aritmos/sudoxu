use super::num::Num;

mod fmt;

/// A cell within the grid, holding information about its known value or possible candidates.
/// Internally represented by a u16.
///
/// # u16 Representation
/// ```txt
/// (000 000) 0 0 0  0 0 0  0 0 0  0
///  ^^^^^^^  ^^^^^^^^^^^^^^^^^^^  ^
///  (banned  candidates           known bit
///  bits)    9 8 7  6 5 4  3 2 1
/// ```
/// ## Known Bit
/// The first bit is the _known bit_.
/// When it is set to 1 the cell is assumed to have a known Number (as opposed to multiple
/// candidates). This means there must only be a single 1 within the following 9 bits.
///
/// ## Candidates
/// The following 9 bits are the _candidate_ bits, these represent the possible candidate numbers
/// 1..=9 that can go in the cell.
///
/// ## Banned Bits
/// The final 6 bits are unused in the current implementation.
/// They must always set to zero. If not then errors are created.
///
/// # Nomenclature
/// We refer to a cell holding a known number as having a _value_ of said number,
/// while a cell not holding a known number as having _candidates_.
/// - `Cell(0b000000_100_000_000_1)` has _value_ 9.
/// - `Cell(0b000000_000_000_111_0)` has _candidates_ 1, 2, and 3.
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell(u16);

#[derive(Debug)]
pub enum CandidateError {
    ParseError,
    BannedBits,             // Cell >= 1024 (= 2^10)
    KnownNoNum,             // Cell == 1
    KnownMultipleNum,       // Cell % 2 == 1 && !(Cell - 1).is_power_of_two()
    NoCandidates,           // Cell == 0
    MultipleSoleCandidates, // Multiple single candidates within sections
}

/// Option<Num> to signify zero or one sole candidates
/// Err if multiple or no possible candidates
// pub type CandidateResult = Result<Option<Num>, CandidateError>;

impl Default for Cell {
    fn default() -> Self {
        Self(0b0000001111111110)
        //           987654321
    }
}

impl Cell {
    /// Creates a new Cell, which can either contain multiple candidates or be known.
    /// Errors if provided with a single candidate but the _known bit_ is not set.
    pub fn new(n: u16) -> Result<Self, CandidateError> {
        // u16 cant be too small or too large
        match n {
            0 => return Err(CandidateError::NoCandidates),
            1024.. => return Err(CandidateError::BannedBits),
            _ => (),
        }

        // unknown cell
        if n % 2 != 1 {
            return Ok(Self(n));
        }

        // known cell
        match n.count_ones() {
            0 => unreachable!(),
            1 => Err(CandidateError::KnownNoNum),
            2 => Ok(Self(n)),
            3.. => Err(CandidateError::KnownMultipleNum),
        }
    }

    /// Creates a new `Cell`, which can either contain multiple candidates or be known.
    /// Does no checking to verify the representation of the `u16` matches that of a `Cell`
    /// # Safety
    /// The caller must ensure that the content of the `u16` is a valid representation of a `Cell`.
    pub unsafe fn new_unchecked(n: u16) -> Self {
        Self(n)
    }

    /// Creates a `Cell` with a known value from a `Num`.
    pub fn new_known(n: Num) -> Cell {
        let inner: u16 = (1 << u16::from(n)) | 1;
        // Safety is guaranteed by construction
        unsafe { Cell::new_unchecked(inner) }
    }

    /// Equivalent functionality to dereferncing.
    /// Used to better showcase intent within the code.
    #[inline]
    pub fn to_u16(self) -> u16 {
        self.0
    }

    /// Returns whether the known bit is set.
    #[inline]
    pub fn is_known(&self) -> bool {
        self.0 & 1 != 0
    }
}
