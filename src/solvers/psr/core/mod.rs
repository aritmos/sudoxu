/// Functions that remove candidates from [Cells](structs::cell::Cell) and encompasing types.
#[doc(hidden)]
pub mod filters;

/// Functions that identify known values within [Cells](structs::cell::Cell).
#[doc(hidden)]
pub mod finders;

/// Types and implementations for the PSR solver category.
#[doc(hidden)]
pub mod structs;
#[doc(inline)]
pub use structs::*;
