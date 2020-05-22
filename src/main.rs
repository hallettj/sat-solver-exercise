mod constants;
mod position;
pub mod puzzle;
pub mod solver;

use crate::solver::sudoku;
use varisat::solver::Solver;

fn main() {
    let mut solver = Solver::new();

    sudoku(&mut solver);

    let solution = solver.solve().unwrap();
    let model = solver.model();
    println!("solution: {:?}, model: {:?}", solution, model);
}
