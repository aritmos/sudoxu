mod simple;

use crate::structs::grid::Grid;

pub trait Solver {
    fn init(grid: Grid) -> Self;
    fn solve(self) -> Grid;
}
