mod fmt;

use super::{
    cell::Cell,
    idx::{GridIdx, SectionIdx},
    section::SectionKind,
};

/// The sudoku Grid.
///
/// # Representation Correctness
/// It is okay for `Cell`s within the Grid to contain candidates that are not allowed due to
/// neighboring known cells. This just means that some Cell's might have not been updated upon
/// finding a value for a cell.
pub struct Grid(pub(super) [Cell; 81]);

#[derive(Debug)]
pub enum GridError {
    FromStringError, // Error in parsing a `String` into a `Grid`
}

// Manual implementation as we cannot derive Default with an inner array of size > 32.
impl Default for Grid {
    fn default() -> Self {
        Self([Cell::default(); 81])
    }
}

impl Grid {
    /// Get the Cell at index `idx`
    #[inline(always)]
    pub fn get(&self, idx: GridIdx) -> Cell {
        self.0[idx]
    }

    pub fn get_mut(&mut self, idx: GridIdx) -> &mut Cell {
        &mut self.0[idx]
    }
}

// Section Related
impl Grid {
    pub fn get_section_grididxs(k: SectionKind, i: SectionIdx) -> [GridIdx; 9] {
        const SECTION_GRIDIDXS: [[usize; 9]; 27] = [
            // Rows
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24, 25, 26],
            [27, 28, 29, 30, 31, 32, 33, 34, 35],
            [36, 37, 38, 39, 40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49, 50, 51, 52, 53],
            [54, 55, 56, 57, 58, 59, 60, 61, 62],
            [63, 64, 65, 66, 67, 68, 69, 70, 71],
            [72, 73, 74, 75, 76, 77, 78, 79, 80],
            // Columns
            [0, 9, 18, 27, 36, 45, 54, 63, 72],
            [1, 10, 19, 28, 37, 46, 55, 64, 73],
            [2, 11, 20, 29, 38, 47, 56, 65, 74],
            [3, 12, 21, 30, 39, 48, 57, 66, 75],
            [4, 13, 22, 31, 40, 49, 58, 67, 76],
            [5, 14, 23, 32, 41, 50, 59, 68, 77],
            [6, 15, 24, 33, 42, 51, 60, 69, 78],
            [7, 16, 25, 34, 43, 52, 61, 70, 79],
            [8, 17, 26, 35, 44, 53, 62, 71, 80],
            // Boxes
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
        let usize_section_grididxs = SECTION_GRIDIDXS[k as usize * 9 + usize::from(i)];
        unsafe { std::mem::transmute(usize_section_grididxs) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn section_grididxs() {
        use super::SectionKind::*;
        let grididxs = Grid::get_section_grididxs(Row, SectionIdx::try_from(0_usize).unwrap());
        let expect = [0_usize, 1, 2, 3, 4, 5, 6, 7, 8].map(|i| GridIdx::try_from(i).unwrap());
        assert_eq!(grididxs, expect);

        let grididxs = Grid::get_section_grididxs(Column, SectionIdx::try_from(2_usize).unwrap());
        let expect =
            [2_usize, 11, 20, 29, 38, 47, 56, 65, 74].map(|i| GridIdx::try_from(i).unwrap());
        assert_eq!(grididxs, expect);

        let grididxs = Grid::get_section_grididxs(Box, SectionIdx::try_from(8_usize).unwrap());
        let expect =
            [60_usize, 61, 62, 69, 70, 71, 78, 79, 80].map(|i| GridIdx::try_from(i).unwrap());
        assert_eq!(grididxs, expect);
    }
}
