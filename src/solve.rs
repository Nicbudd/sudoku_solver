pub mod trad_solver;
pub mod z3_solver;

pub trait Solve {
    fn solve(&mut self);
}