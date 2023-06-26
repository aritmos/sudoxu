//! Identifying cells that are known

/// When a `Cell` only contains one candidate
mod single_candidate;
pub use single_candidate::*;

/// When within a row, column or square a `Cell` contains the only candidate for a certain `Num`
mod unique_candidate;
pub use unique_candidate::*;
