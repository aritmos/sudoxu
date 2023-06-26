use crate::structs::*;
use std::mem::{transmute, MaybeUninit};

#[derive(Clone, Copy)]
pub struct Square([Cell; 9]);

// A square whose rows or columns have been collapsed
// into one by folding with BITWISE OR
#[derive(Clone, Copy)]
pub struct SubSection([Cell; 3]);

impl Square {
    /// Folds the rows and columns of a square using `OR`
    /// returns [rows, cols]
    pub fn fold_into_subsections(self, rotate: bool) -> [Cell; 3] {
        let rows = [
            self.0[0] | self.0[1] | self.0[2],
            self.0[3] | self.0[4] | self.0[5],
            self.0[6] | self.0[7] | self.0[8],
        ];
        let cols = [
            self.0[0] | self.0[3] | self.0[6],
            self.0[1] | self.0[4] | self.0[7],
            self.0[2] | self.0[5] | self.0[8],
        ];
        if !rotate {
            rows
        } else {
            cols
        }
    }
}

impl From<SubSection> for [Cell; 3] {
    fn from(value: SubSection) -> Self {
        value.0
    }
}

impl SubSection {
    /// Takes the three subsections of a square and a number `0 <= N <= 3`.
    /// Within each bit stores a `1` if we have seen N `1`s at that bit position,
    /// else stores a `0`.
    /// EXAMPLE: (using smaller "`u4`s")
    ///
    ///          `// Align these vertically and count the 1s in each column`
    ///          `bits = [0b0101, 0b0011, 0b0010];`
    ///          
    ///          `assert!(bits.contain_count<0>() = 0b1000);`
    ///          `assert!(bits.contain_count<1>() = 0b0100);`
    ///          `assert!(bits.contain_count<2>() = 0b0011);`
    ///          `assert!(bits.contain_count<3>() = 0b0000);`
    /// COMMENT: N should technically be represented as an `Idx<4>`,
    ///          but this is not allowed as a const generic.
    pub fn contain_count<const N: u8>(&self) -> Cell {
        let [x, y, z] = self.0;

        let bit_and = x & y & z;
        let bit_or = x | y | z;
        let bit_xor = x ^ y ^ z;

        match N {
            0 => !bit_or,
            1 => bit_xor & !bit_and,
            2 => bit_or & !bit_or,
            3 => bit_and,
            4.. => unreachable!(),
        }
    }
}

/// Represents a collection of 3 squares.
/// COMMENT: The squares are thought to be horizontal.
/// .        If the are represents a vertical alignment of squares
/// .        they are rotated counterclockwise about the bottom.
/// COMMENT: beware that `grid.get_area()` utilises unitialised forms of `Area`.
/// .        Any modification to the fields of `Area` should be immediatelly matched
/// .        within said method.
/// (possibly rotated) squares used in construction:
/// `Cell Cell Cell | Cell Cell Cell | Cell Cell Cell`
/// `Cell Cell Cell | Cell Cell Cell | Cell Cell Cell`
/// `Cell Cell Cell | Cell Cell Cell | Cell Cell Cell`
/// data: combine the candidates of each column into one
///       (using `OR` on columns)
/// `Cell | Cell | Cell`
/// `Cell | Cell | Cell`
/// `Cell | Cell | Cell`
/// known: known numbers within the square.
/// .      represented as candidates in a `Cell`.
/// `Cell | Cell | Cell`
/// masks: bitmasks for each of the 9 areas.
/// .      (0..=2 for the first square and so on)
/// equality: check pairs of columns in data for equality
///           ordering is: `0 <-> 1, 0 <-> 2, 1 <-> 2`
///           (using `!XOR` on columns then `AND` on rows)
/// `[Cell, Cell, Cell]`
/// `Cell | Cell | Cell`
/// count_1_vertical: like `count_1` but for `data_vertical` (horizontal folding)
/// count_2: collapse data vertically by counting if bits appear twice in a column
/// `Cell | Cell | Cell`
pub struct Area {
    pub values: [SubSection; 3],
    pub masks: [[Cell; 3]; 3],
    pub known: [Cell; 3],
    pub comparisons: [Cell; 3], // bitwise equality between columns: 0-1, 0-2, 1-2
    pub count_1: [Cell; 3],     // contain_count<1> on each collapsed square
    pub count_2: [Cell; 3],     // contain_count<2> on each collapsed square
}

/// `0   1   2`
/// `|   |   |`
/// `v   v   v`
/// `S | S | S <- 3`
/// `- + - + -`
/// `S | S | S <- 4`
/// `- + - + -`
/// `S | S | S <- 5`
pub type AreaIdx = Idx<6>;

impl Area {
    /// Updates `values` using `masks`
    /// TODO: current implementation masks the entire area this is somewhat
    /// .     inefficient. Perhaps I should implement a method where only modified
    /// .     sectors get written to as to not suffer performance in the late game.
    /// .     where we expect few modifications.
    fn update_values(&mut self) {
        self.values
            .iter_mut()
            .zip(self.masks)
            .for_each(|(subsection, masks)| {
                for (cell, mask) in subsection.0.iter_mut().zip(masks) {
                    cell.mask(mask);
                }
            });
    }

    /// Updates `comparisons`, `count_1` and `count_2` using `data`
    fn update_data(&mut self) {
        fn collapsed_square_equality(a: SubSection, b: SubSection) -> Cell {
            let (a, b) = (a.0, b.0); // use inner values
            !(a[0] ^ b[0]) & !(a[1] ^ b[1]) & !(a[2] ^ b[2])
        }

        let data = self.values;

        self.comparisons = [
            collapsed_square_equality(data[0], data[1]),
            collapsed_square_equality(data[0], data[2]),
            collapsed_square_equality(data[1], data[2]),
        ];

        // compute counts
        self.count_1 = data.map(|s| s.contain_count::<1>());
        self.count_2 = data.map(|s| s.contain_count::<2>());
    }
}

impl Grid {
    /// Obtains the `Area` from within the `Grid`, rotating if necessary
    /// COMMENT: CONTAINS VERY UNSAFE CODE!
    /// .        Upon changes always make sure that the unitialised `Area`
    /// .        at the end gets all of its fields properly initialised.
    pub fn get_area(&self, n: AreaIdx) -> Area {
        let rotate = u8::from(n) < 3;
        // get the cells in the 3 squares
        let square_idxs: [SectionIdx; 3] = unsafe {
            std::mem::transmute(match u8::from(n) {
                0 => [0_u8, 3, 6],
                1 => [1, 4, 7],
                2 => [2, 5, 8],
                3 => [0, 1, 2],
                4 => [3, 4, 5],
                5 => [6, 7, 8],
                _ => unreachable!(),
            })
        };

        let squares = square_idxs.map(|i| {
            let idxs = Grid::square_indices(i);
            Square(self.get_cells(idxs))
        });

        let values = squares.map(|s| SubSection(s.fold_into_subsections(rotate)));
        let known = squares.map(|s| Cell::combine_candidates(&s.0));

        // VERY UNSAFE!
        // Done so we can use `update_data` without needing to instantiate the fields
        // Verify upon changes that every field is initialised!
        #[allow(clippy::let_and_return)]
        let area = {
            let mut uninit_area: Area = unsafe { transmute(MaybeUninit::<Area>::uninit()) };
            uninit_area.values = values;
            uninit_area.masks = [[Cell::default(); 3]; 3];
            uninit_area.known = known;
            uninit_area.update_data();
            uninit_area
        };

        area
    }
}
