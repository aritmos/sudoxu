use crate::structs::*;
use std::mem::transmute;

#[derive(Clone, Copy)]
pub struct Section([Cell; 9]);

pub type Square = Section;

pub enum SectionType {
    Row,
    Column,
    Square,
}

/// Wrapper for `[Cell; 9]`, representing rows, columns, and squares.
impl Section {
    pub fn new(cells: [Cell; 9]) -> Self {
        unsafe { transmute(cells) }
    }

    pub fn to_cells(self) -> [Cell; 9] {
        unsafe { transmute(self) }
    }

    pub fn to_string(self, section_type: SectionType) -> String {
        match section_type {
            SectionType::Row | SectionType::Column => format!(
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
            SectionType::Square => format!(
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
                n = 11
            ),
        }
    }
}
