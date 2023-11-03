use super::num::Num;

mod fmt;
mod update;
#[doc(inline)]
pub use update::*;

/// A cell within the grid.
/// Holds information about what candidates the cell has, and if the value of the cell is known.
///
/// [`Cell`]s are wrappers for `u16`s.
///
/// In general, this wrapping is mostly transparent, and in many cases [`Cell`]s can be almost
/// interpreted as type aliases for `u16`s.
/// That being said, not all operations on `u16`s are valid or safe in the context of a [`Cell`]'s
/// representation. For this reason [`Cell`] does not implement [`Deref<u16>`](std::ops::Deref),
/// nor is its inner `u16` marked public. In many occasions it is still necessary to access the
/// underyling `u16`. For this reason [`Cell`] contains an explicit [`to_u16`](Cell::to_u16)
/// method for casting.
///
/// # Internal Representation
///
/// ```txt
/// 0b  000000  0 0 0  0 0 0  0 0 0  0
///     ^^^^^^  ^^^^^^^^^^^^^^^^^^^  ^
///     |       |                    known bit
///     |       candidates
///     |      (9 8 7  6 5 4  3 2 1)
///     "banned" bits
/// ```
///
/// Because of this representation, for visual clarity the unusual byte grouping of
/// "`0b000000_000_000_000_0`" is commonly used for `u16`s relating to `Cells`.
///
/// Errors relating to `Cell` representations (see below) are contained in the [`CandidateError`] enum.
///
/// #### Known Bit
/// The first bit is the "known bit".
/// When it is set, the cell is assumed to have a known number (the cell has been filled out),
/// as opposed to multiple candidates (having pencilmarks).
/// This means there must only be a single "candidate bit" set.
/// ```
/// // Valid u16 representation of a cell with value 9.
/// assert!(Cell::new(0b000000_100_000_000_1).is_ok());
///
/// // Invalid u16 representation. Known bit is set but candidate bits for 5 and 6 are set.
/// // This would represent a cell with a known value that is both 5 and 6.
/// assert!(Cell::new(0b000000_000_110_000_1).is_err());
///
/// // Invalid u16 representation. Known bit is set but there are no set candidate bits.
/// // This would represent a cell with a known value, but no known value is set.
/// assert!(Cell::new(0b000000_000_000_000_1).is_err());
/// ```
/// #### Candidates
/// The following 9 bits are the "candidate" bits, these represent the possible candidate numbers
/// 1..=9 that can go in the cell.
///
/// #### Banned Bits
/// The final 6 bits are unused in the current implementation.
/// If they are not all set to zero, then the `Cell`s representation is deemed invalid.
///
/// ```
/// // Valid u16 representation of a cell with candidates 1, 2, and 3.
/// assert!(Cell::new(0b000000_000_000_111_0).is_ok());
///
/// // Invalid u16 representation of a cell. Banned bits are not all zero.
/// assert!(Cell::new(0b001100_000_000_111_0).is_err());
/// ```
///
/// # Safety
/// TODO
///
/// # Nomenclature
/// We refer to a cell holding a known number as having a _value_ of said number,
/// while a cell not holding a known number as having _candidates_:
/// ```
/// Cell(0b000000_100_000_000_1); // has "value" 9.
/// Cell(0b000000_000_000_111_0); // has "candidates" 1, 2, and 3.
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell(u16);

/// Errors relating to a Cell's candidates.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CandidateError {
    /// Bits set within the banned sector.
    /// ```
    /// let cell = unsafe{ Cell::new_unchecked(0b000011_000_001_000_1) };
    /// assert!(cell.to_u16() >= 1024); // 1024 = 2^10
    /// ```
    BannedBits,
    /// Known bit is set but no candidates are set.
    ///
    /// ```
    /// let cell = unsafe{ Cell::new_unchecked(0b000000_000_000_000_1) };
    /// assert!(cell.to_u16() == 1);
    /// ```
    KnownNoNum,
    /// Known bit is set but multiple candidates are also set.
    /// ```
    /// let cell = unsafe{ Cell::new_unchecked(0b000000_111_000_000_1) };
    /// assert!(cell.to_u16() % 2 == 1 && !(cell.to_u16() & 0b000000_111_111_111_0 - 1).is_power_of_two());
    /// ```
    KnownMultipleNum,
    /// Cell contains no candidates/value.
    ///
    /// ```
    /// let cell = unsafe{ Cell::new_unchecked(0b000000_000_000_000_0) };
    /// assert!(cell.to_u16() & 0b000000_111_111_111_0 == 0);
    /// ```
    NoCandidates,
    /// Cell has multiple unique candidates within its [Sections](super::section::Section).
    /// This is not an error in the representation of the [Cell], but an error in the state of the
    /// [Grid](super::grid::Grid).
    ///
    /// See [finders::unique_candidate](super::super::finders::unique_candidate) for more
    /// information.
    MultipleUniqueCandidates,
}

impl Default for Cell {
    fn default() -> Self {
        Self(0b000000_111_111_111_0)
        //            987 654 321
    }
}

/// Base Implementations
impl Cell {
    /// Checks if a given [`Cell`] has an allowed representation.
    pub fn check(self) -> Result<(), CandidateError> {
        let self_u16 = self.to_u16();
        if self_u16 == 0 {
            return Err(CandidateError::NoCandidates);
        }
        if self_u16 >= 1024 {
            return Err(CandidateError::BannedBits);
        }
        if self_u16 % 2 != 1 {
            return Ok(());
        }
        match self_u16.count_ones() {
            0 => unreachable!(),
            1 => Err(CandidateError::NoCandidates),
            2 => Ok(()),
            _ => Err(CandidateError::KnownMultipleNum),
        }
    }

    /// Creates a new [`Cell`] from an inner `u16`. Returns a [`CandidateError`] if the `u16`'s
    /// does not follow the [allowed representation](Cell#internal-representation).
    pub fn new(n: u16) -> Result<Self, CandidateError> {
        let cell = unsafe { Self::new_unchecked(n) };
        cell.check().map(|_| cell)
    }

    /// Creates a new `Cell`, which can either contain multiple candidates or be known.
    /// Does no checking to verify the representation of the `u16` matches that of a `Cell`
    /// # Safety
    /// The caller must ensure that the content of the `u16` is a
    /// [valid representation](Cell#internal-representation) of a `Cell`.
    #[inline(always)]
    pub unsafe fn new_unchecked(n: u16) -> Self {
        Self(n)
    }

    /// Creates a [Cell] with a known value from a [Num].
    pub fn new_known(n: Num) -> Cell {
        let inner: u16 = (1 << u16::from(n)) | 1;
        // Safety is guaranteed by construction
        unsafe { Cell::new_unchecked(inner) }
    }

    /// Returns the inner `u16` within the [Cell].
    // #[inline(always)]
    pub fn to_u16(self) -> u16 {
        self.0
    }

    /// Returns whether the known bit is set.
    #[inline(always)]
    pub fn is_known(self) -> bool {
        self.0 & 1 != 0
    }

    /// Wrapper for [`Cell::new_unchecked(0)`](Cell::new_unchecked)
    /// # Safety
    /// **This method intentionally returns an invalid `Cell` representation** meant
    /// to be **only** used within internal calculations or accumulations.
    /// The caller guarantees that this form of a `Cell` is only used within such cases
    /// and is never returned or used as an actual `Cell`.
    #[inline(always)]
    pub unsafe fn zerod() -> Cell {
        Cell::new_unchecked(0_u16)
    }
}
