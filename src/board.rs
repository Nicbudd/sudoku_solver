use std::{fs, path::Path};
use std::fmt::{self, Display, Debug};
use anyhow::Result;
use crate::Rules;

pub trait Board: Display + Clone + Debug + Eq {
    fn completed_cells(&self) -> [u8; 81];
    fn rules(&self) -> &Rules;

    fn row(&self, row: usize) -> [u8; 9] {
        let start = row*9;
        self.completed_cells()[start..start+9].try_into().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct BasicBoard {
    candidates: [[bool; 9]; 81],
    cells: [u8; 81],
    rules: Rules,
}

impl Board for BasicBoard {
    fn completed_cells(&self) -> [u8; 81] {
        self.cells
    }
    fn rules(&self) -> &Rules {
        &self.rules
    }
}

impl Eq for BasicBoard {}
impl PartialEq for BasicBoard {
    fn eq(&self, other: &Self) -> bool {
        self.candidates == other.candidates
    }
}

impl Display for BasicBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cells.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""))
    }
}
