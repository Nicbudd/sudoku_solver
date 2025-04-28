pub mod sudoku;

#[derive(Clone, Debug)]
pub struct Rules {
    pub normal_sudoku: bool,
    pub rubiks: bool,
    pub sets: Vec<u128>
}