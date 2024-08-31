mod columns;
mod crates;

use std::fmt::{Display, Write};

pub use columns::*;
pub use crates::*;

use crate::models::Section;

pub const REGULAR_SECTIONS: [Section; 5] =
    [Section::A, Section::B, Section::C, Section::D, Section::E];

pub const SINGLE_CRATE_SECTIONS: [Section; 2] = [Section::G, Section::H];

pub const TWO_BY_X_SECTION: [Section; 1] = [Section::F];

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Section::D => "D",
            Section::E => "E",
            Section::F => "F",
            Section::G => "G",
            Section::H => "H",
        };
        f.write_str(letter)
    }
}
