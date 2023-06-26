use crate::structs::*;
use std::{
    fmt::Display,
    mem::{transmute, MaybeUninit},
};

pub enum SectionType {
    Row,
    Column,
    Square,
}

pub struct Section([Cell; 9]);

impl From<[Cell; 9]> for Section {
    fn from(cells: [Cell; 9]) -> Self {
        Self(cells)
    }
}

impl Section {
    pub fn pop(cells: Self, idx: SectionIdx) -> [Cell; 8] {
        // COMMENT: It seems like using maybeuninit has marginal assembly differences
        //          (https://godbolt.org/z/MbvrdTz6c) compared to simply instantiating
        //          the array manually.
        // COMMENT: Currently it is not allowed to do a transmute on a const len array
        //          So I can't make this function generic in the form:
        //          `fn([Cell; N], Idx<N>) -> [Cell; N - 1]`
        let mut remaining_cells: [MaybeUninit<Cell>; 8] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut iter = cells.0.into_iter();
        for (i, c) in remaining_cells.iter_mut().enumerate() {
            let cell = iter.next().unwrap();
            if i != idx.into() {
                *c = MaybeUninit::new(cell);
            }
        }
        unsafe { transmute(remaining_cells) }
    }

    /// Custom `to_string` method instead of the `ToString: Display` one.
    /// TODO: Possibly move the intricate `Square` implementation into the `Square` struct
    pub fn to_str(&self, section_type: SectionType) -> String {
        match section_type {
            SectionType::Square => {
                let n = (self
                    .0
                    .iter()
                    .map(|c| (c.to_u16() & !1).count_ones())
                    .max()
                    .unwrap()
                    + 2) as usize;
                format!(
                    "{:^n$} {:^n$} {:^n$}\n{:^n$} {:^n$} {:^n$}\n{:^n$} {:^n$} {:^n$}",
                    self.0[0].to_string(),
                    self.0[1].to_string(),
                    self.0[2].to_string(),
                    self.0[3].to_string(),
                    self.0[4].to_string(),
                    self.0[5].to_string(),
                    self.0[6].to_string(),
                    self.0[7].to_string(),
                    self.0[8].to_string(),
                )
            }
            _ => format!(
                "{} {} {} {} {} {} {} {} {}",
                self.0[0],
                self.0[1],
                self.0[2],
                self.0[3],
                self.0[4],
                self.0[5],
                self.0[6],
                self.0[7],
                self.0[8]
            ),
        }
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8]
        )
    }
}
