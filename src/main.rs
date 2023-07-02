mod solver;

use std::path::Path;

use solver::*;


fn test_sudoku() -> Board {
    let rules = Rules {normal_sudoku: true, sets: vec![]};


    let mut b = Board::new(rules);

    b.update();

    b
}

fn main() {

    let rules = Rules {normal_sudoku: true, sets: vec![]};

    let mut b = Board::from_file(Path::new("test_sudoku/test.sudoku"), rules).unwrap();

    

    println!("{}", b.solve());

    println!("{}", b);
}

