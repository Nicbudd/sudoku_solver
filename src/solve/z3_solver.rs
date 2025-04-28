use std::task::Context;
use std::vec;

use z3::ast::{self, Ast, Int};

use crate::board::{Board, BasicBoard};
use crate::solve::Solve;

impl Solve for BasicBoard {
    fn solve(&mut self) {

        // init z3 instance
        let ctx = &z3::Context::default();
        let solver = z3::Solver::new(&ctx);

        let mut variables = Vec::new();
        let cells = self.completed_cells();

        for i in 0..9 {
            for j in 0..9 {
                let cell_value = cells[(i*9)+j];
                let cell_name = format!("cell_{i}_{j}");
                let z3cell = Int::new_const(&ctx, cell_name);
                variables.push(z3cell);
                solver.assert(&z3cell.le(&Int::from_i64(&ctx, 9)));
                solver.assert(&z3cell.ge(&Int::from_i64(&ctx, 1)));
            }
        }

        let groups = vec![];

        for i in 0..9 {
            for j in 0..9 {
            }
        }

        solver.assert(&ctx, );

    }
}
