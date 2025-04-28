use std::{fs::File, io::Read, path::Path, time::Instant};
use clap::Parser;


mod rules;
pub use rules::Rules;

mod solve;
pub use solve::*;

mod board;
pub use board::Board;


/// Sudoku solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the .sudoku file or the sudoku string itself
    #[arg(short, long)]
    sudoku: String,

    /// Normal sudoku rules
    #[arg(short, long, default_value_t = true)]
    normal_rules: bool,

    /// Rubik's Cube rules
    #[arg(short, long, default_value_t = false)]
    rubiks_rules: bool,

    /// Stop the solve if we need to bifurcate
    #[arg(long, default_value_t = false)]
    stop_if_bifurcate: bool,
}

fn main() {

    let args = Args::parse();

    let rules = Rules {
        normal_sudoku: args.normal_rules,
        rubiks: args.rubiks_rules, 
        sets: vec![]
    };

    let mut sudoku_string = String::new();

    if let Ok(mut file) = File::open(&args.sudoku) {
        let _ = file.read_to_string(&mut sudoku_string).unwrap();
    } else {
        sudoku_string = args.sudoku.clone();
    }

    let mut brd = solve::trad_solver::BitmaskBoard::from_string(sudoku_string, rules.clone());

    let mut recursion_count = 0;

    let solns = brd.solve(&mut recursion_count);

    if solns == 1 {
        println!("1 solution:\n{}",brd);
    } else {
        println!("Uh oh! {solns} solutions.\n");

    }

    let mut sudoku_string_slice = ['-'; 81];

    // for a in 0..81 {
    //     for b in 0..81 {
    //         for c in 0..81 {
    //             sudoku_string_slice = ['-'; 81];

    //             sudoku_string_slice[a] = '3';
    //             sudoku_string_slice[b] = '1';
    //             sudoku_string_slice[c] = '3';


    //             let sudoku_string = sudoku_string_slice.iter().collect::<String>() + 
    //                     "XXX BOG XXX
    //                     XXX YBW XXX
    //                     XXX RRW XXX
    //                     OGY BWR BGW
    //                     ROO GWG RRB
    //                     OYY GWG WBB
    //                     XXX ROR YYW
    //                     XXX BGR YYW
    //                     XXX GOO YBO";

    //             let mut brd = Board::from_string(sudoku_string, rules.clone());

    //             let count = c + (81*b) + (81*81*a);


    //             let mut recursion_count = 0;
    //             let solns = brd.solve(&mut recursion_count);

    //             if solns == 1 {
    //                 println!("{}\n",brd.short_string());

    //             }

    //             if count % 100 == 0 {
    //                 println!("Attempt {}", count);
    //             }

    //         }
    //     }
    // }

}

