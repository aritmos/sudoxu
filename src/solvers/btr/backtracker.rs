use std::ops::{Index, IndexMut};

use crate::{board::Board, solver::Solver};

/// Backtracking solver
pub struct Backtracker {
    grid: Grid,
}

/// Grid
#[derive(Debug)]
pub struct Grid([u8; 81]);

impl Solver for Backtracker {
    fn init(board: Board) -> Self {
        Self { grid: board.into() }
    }

    fn solve(mut self) -> Board {
        self.grid.backtrack_solve();
        self.grid.into()
    }
}

impl From<Board> for Grid {
    fn from(board: Board) -> Self {
        Self(board.0)
    }
}

impl From<Grid> for Board {
    fn from(grid: Grid) -> Self {
        Self(grid.0)
    }
}

impl Index<usize> for Grid {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

const ROW_INNER_INDICES: [[u8; 9]; 9] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
];
const COL_INNER_INDICES: [[u8; 9]; 9] = [
    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],
];
const SQU_INNER_INDICES: [[u8; 9]; 9] = [
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

impl Grid {
    /// Finds all of the indexes of the cells that are unknown.
    /// Used at the start of the solving.
    pub fn guess_idxs(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| (n == 0).then_some(i))
            .collect()
    }

    /// Solves the grid by backtracking
    pub fn backtrack_solve(&mut self) {
        let mut grid = self;
        let guess_idxs = grid.guess_idxs();
        let mut i: usize = 0;

        while i < guess_idxs.len() {
            let guess_idx = guess_idxs[i];
            grid[guess_idx] += 1;

            if grid[guess_idx] == 10 {
                grid[guess_idx] = 0;
                i = i.checked_sub(1).expect("Backtracked beyond the start");
                continue;
            }

            let row_idx = guess_idx / 9;
            let col_idx = guess_idx % 9;
            let squ_idx = col_idx / 3 + 3 * (row_idx / 3);

            let row_ok = ROW_INNER_INDICES[row_idx]
                .map(|i| grid[i as usize])
                .iter()
                .filter(|&&n| n == grid[guess_idx])
                .count()
                == 1;
            let col_ok = COL_INNER_INDICES[col_idx]
                .map(|i| grid[i as usize])
                .iter()
                .filter(|&&n| n == grid[guess_idx])
                .count()
                == 1;
            let squ_ok = SQU_INNER_INDICES[squ_idx]
                .map(|i| grid[i as usize])
                .iter()
                .filter(|&&n| n == grid[guess_idx])
                .count()
                == 1;

            if row_ok && col_ok && squ_ok {
                i += 1;
            }
        }
    }
}
