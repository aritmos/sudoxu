//! Filters are methods to remove candidates within Cells inside of the Grid.

/// Filter the neighboring cells when a `Cell` becomes known
mod filter_known;
pub use filter_known::*;

/// Filters on adjacent squares based on candidate positioning
/// (See medium techniques: "candidate lines", "double pairs", "multiple lines")
mod filter_lines;
pub use filter_lines::*;
