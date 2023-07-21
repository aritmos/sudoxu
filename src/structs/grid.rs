use crate::structs::*;
use crate::utils::array_pop;
use std::mem::{transmute, MaybeUninit};

// #[derive(Clone, Copy)]
pub struct Grid([Cell; 81]);

// border chars: ─ │ ┌ ┐ ┘ └ ┼
/// 00 01 02 │ 03 04 05 │ 06 07 08
/// 09 10 11 │ 12 13 14 │ 15 16 17
/// 18 19 20 │ 21 22 23 │ 24 25 26
/// ─────────┼──────────┼─────────
/// 27 28 29 │ 30 31 32 │ 33 34 35
/// 36 37 38 │ 39 40 41 │ 42 43 44
/// 45 46 47 │ 48 49 50 │ 51 52 53
/// ─────────┼──────────┼─────────
/// 54 55 56 │ 57 58 59 │ 60 61 62
/// 63 64 65 │ 66 67 68 │ 69 70 71
/// 72 73 74 │ 75 76 77 │ 78 79 80
pub type GridIdx = Idx<81>;

/// # Row and column `SectionIdx`s are the usual row and column numbers or the `Grid`
/// # Square `SectionIdx`s:
/// 0 │ 1 │ 2
/// ──┼───┼──
/// 3 │ 4 │ 5
/// ──┼───┼──
/// 6 │ 7 │ 8
/// # Inner square `SectionIdx`s:
/// 0 1 2 │ 0 1 2 │ 0 1 2
/// 3 4 5 │ 3 4 5 │ 3 4 5
/// 6 7 8 │ 6 7 8 │ 6 7 8
/// ──────┼───────┼──────
/// 0 1 2 │ ...
pub type SectionIdx = Idx<9>;

impl Default for Grid {
    fn default() -> Self {
        Self([Cell::default(); 81])
    }
}

impl Grid {
    /// Get the Cell at index `idx`
    #[inline(always)]
    pub fn get(&self, idx: GridIdx) -> Cell {
        self.0[usize::from(idx)]
    }

    pub fn get_mut(&mut self, idx: GridIdx) -> &mut Cell {
        &mut self.0[usize::from(idx)]
    }
}

// border chars: ─ │ ┌ ┐ ┘ └ ┼

/// Grid Display:
///┌───────────────────────┐
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ ──────┼───────┼────── │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ ──────┼───────┼────── │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///│ 0 0 0 │ 0 0 0 │ 0 0 0 │
///└───────────────────────┘
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_string = String::new();
        fn row_to_string(grid: &Grid, n: SectionIdx) -> String {
            let row = grid.get_cells(Grid::row_indices(n)).map(char::from);
            format!(
                "│ {} {} {} │ {} {} {} │ {} {} {} │\n",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8]
            )
        }

        out_string += "┌───────────────────────┐\n";
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(0_u8) });
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(1_u8) });
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(2_u8) });
        out_string += "│ ──────┼───────┼────── │\n";
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(3_u8) });
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(4_u8) });
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(5_u8) });
        out_string += "│ ──────┼───────┼────── │\n";
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(6_u8) });
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(7_u8) });
        out_string += &row_to_string(self, unsafe { Idx::new_unchecked(8_u8) });
        out_string += "└───────────────────────┘\n";

        write!(f, "{out_string}")
    }
}

impl Grid {
    pub fn row_indices(n: SectionIdx) -> [GridIdx; 9] {
        let arr: [u8; 9] = match u8::from(n) {
            0 => [0, 1, 2, 3, 4, 5, 6, 7, 8],
            1 => [9, 10, 11, 12, 13, 14, 15, 16, 17],
            2 => [18, 19, 20, 21, 22, 23, 24, 25, 26],
            3 => [27, 28, 29, 30, 31, 32, 33, 34, 35],
            4 => [36, 37, 38, 39, 40, 41, 42, 43, 44],
            5 => [45, 46, 47, 48, 49, 50, 51, 52, 53],
            6 => [54, 55, 56, 57, 58, 59, 60, 61, 62],
            7 => [63, 64, 65, 66, 67, 68, 69, 70, 71],
            8 => [72, 73, 74, 75, 76, 77, 78, 79, 80],
            _ => unreachable!(),
        };

        unsafe { transmute(arr) }
    }

    pub fn col_indices(n: SectionIdx) -> [GridIdx; 9] {
        let arr: [u8; 9] = match u8::from(n) {
            0 => [0, 9, 18, 27, 36, 45, 54, 63, 72],
            1 => [1, 10, 19, 28, 37, 46, 55, 64, 73],
            2 => [2, 11, 20, 29, 38, 47, 56, 65, 74],
            3 => [3, 12, 21, 30, 39, 48, 57, 66, 75],
            4 => [4, 13, 22, 31, 40, 49, 58, 67, 76],
            5 => [5, 14, 23, 32, 41, 50, 59, 68, 77],
            6 => [6, 15, 24, 33, 42, 51, 60, 69, 78],
            7 => [7, 16, 25, 34, 43, 52, 61, 70, 79],
            8 => [8, 17, 26, 35, 44, 53, 62, 71, 80],
            _ => unreachable!(),
        };

        unsafe { transmute(arr) }
    }

    pub fn square_indices(n: SectionIdx) -> [GridIdx; 9] {
        let arr: [u8; 9] = match u8::from(n) {
            0 => [0, 1, 2, 9, 10, 11, 18, 19, 20],
            1 => [3, 4, 5, 12, 13, 14, 21, 22, 23],
            2 => [6, 7, 8, 15, 16, 17, 24, 25, 26],
            3 => [27, 28, 29, 36, 37, 38, 45, 46, 47],
            4 => [30, 31, 32, 39, 40, 41, 48, 49, 50],
            5 => [33, 34, 35, 42, 43, 44, 51, 52, 53],
            6 => [54, 55, 56, 63, 64, 65, 72, 73, 74],
            7 => [57, 58, 59, 66, 67, 68, 75, 76, 77],
            8 => [60, 61, 62, 69, 70, 71, 78, 79, 80],
            _ => unreachable!(),
        };
        unsafe { transmute(arr) }
    }

    /// Returns the `SectionIdx`s of a given GridIdx in the order:
    /// `[Row, Column, Square, InnerSquare]`.
    ///
    /// # Comment
    /// "`InnerRow`" and "`InnerCol`" are not necessary as these
    /// are simply the values of `Column` and `Row`.
    ///
    /// # Example (See `GridIdx`; casts to `Idx` are omitted for clarity)
    /// ```rust
    /// // the Cell with GridIdx 50 is in: (all 0 indexed)
    /// // Row 5, Column 6, Square 4, index 8 within the square (InnerSquare)
    /// assert_eq!(
    ///     Grid::section_indices(50),
    ///     [5, 6, 4, 8]
    /// )
    /// ```
    pub fn section_indices(idx: GridIdx) -> [SectionIdx; 4] {
        let idx: u8 = idx.into();
        let row_idx = idx / 9;
        let col_idx = idx % 9;
        let square_idx = 3 * (row_idx / 3) + (col_idx / 3);
        let inner_square_idx = { 3 * (row_idx % 3) + (col_idx % 3) };

        [row_idx, col_idx, square_idx, inner_square_idx]
            .map(|i| unsafe { SectionIdx::new_unchecked(i) })
    }

    /// Returns the complimentary indices within the cells row, column and square. (all neighboring
    /// cell's indices)
    /// # Example (see `GridIdx`; the casts to `Idx` are excluded for clarity)
    /// ```rust
    /// assert_eq!(
    ///     Grid::compliment_indices(40),
    ///     [
    ///         [36, 37, 38, 39, 41, 42, 43, 44],
    ///         [04, 13, 22, 31, 49, 58, 67, 76],
    ///         [30, 31, 32, 39, 41, 48, 49, 50],
    ///     ]
    /// )
    /// ```
    pub fn compliment_indices(idx: GridIdx) -> [[GridIdx; 8]; 3] {
        let [row_idx, col_idx, square_idx, _] = Grid::section_indices(idx);

        let row_idxs = Grid::row_indices(row_idx);
        let col_idxs = Grid::col_indices(col_idx);
        let square_idxs = Grid::square_indices(square_idx);

        let row_comp_idxs = array_pop(row_idxs, col_idx);
        let col_comp_idxs = array_pop(col_idxs, row_idx);
        let square_comp_idxs = array_pop(square_idxs, square_idx);

        [row_comp_idxs, col_comp_idxs, square_comp_idxs]
    }
}

impl Grid {
    /// Obtain (a copy of) the cells given a slice of indices into the grid
    pub fn get_cells<const N: usize>(&self, indices: [GridIdx; N]) -> [Cell; N] {
        let mut cells: [MaybeUninit<Cell>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for (cell, i) in cells.iter_mut().zip(indices) {
            *cell = MaybeUninit::new(self.get(i));
        }
        unsafe { MaybeUninit::array_assume_init(cells) }
    }
}
