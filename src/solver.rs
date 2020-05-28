use crate::constants::PUZZLE_ISIZE;
use crate::position::PositionWithValue;
use crate::puzzle::Puzzle;
use std::{error, fmt};
use varisat::solver::{Solver, SolverError};
use varisat::{CnfFormula, ExtendFormula, Lit};

pub fn solve_puzzle(puzzle: &Puzzle) -> Result<Puzzle, PuzzleSolverError> {
    let mut solver = Solver::new();
    sudoku(&mut solver);
    solver.assume(&puzzle.to_model());
    let has_solution = solver.solve()?;
    if !has_solution {
        Err(PuzzleSolverError::NotSatisfiable)
    } else {
        let model = solver.model().ok_or(PuzzleSolverError::NoModel)?;
        Ok(Puzzle::from_model(&model))
    }
}

fn sudoku(solver: &mut Solver) {
    no_row_contains_duplicate_numbers(solver);
    no_column_contains_duplicate_numbers(solver);
    no_3x3_boxes_contain_duplicate_numbers(solver);
    each_position_contains_exactly_one_number(solver);
}

fn no_row_contains_duplicate_numbers(solver: &mut Solver) {
    for row in 1..=PUZZLE_ISIZE {
        for value in 1..=PUZZLE_ISIZE {
            let literals: Vec<_> = (1..=PUZZLE_ISIZE)
                .map(|column| Lit::from(&PositionWithValue { row, column, value }))
                .collect();
            solver.add_formula(&exactly_one(&literals))
        }
    }
}

fn no_column_contains_duplicate_numbers(solver: &mut Solver) {
    for column in 1..=PUZZLE_ISIZE {
        for value in 1..=PUZZLE_ISIZE {
            let literals: Vec<_> = (1..=PUZZLE_ISIZE)
                .map(|row| Lit::from(&PositionWithValue { row, column, value }))
                .collect();
            solver.add_formula(&exactly_one(&literals))
        }
    }
}

fn no_3x3_boxes_contain_duplicate_numbers(solver: &mut Solver) {
    for box_row in 0..3 {
        for box_column in 0..3 {
            for value in 1..=PUZZLE_ISIZE {
                let literals: Vec<_> = (1..=3)
                    .flat_map(move |row| {
                        (1..=3).map(move |column| {
                            Lit::from(&PositionWithValue {
                                row: row + (box_row * 3),
                                column: column + (box_column * 3),
                                value,
                            })
                        })
                    })
                    .collect();
                solver.add_formula(&exactly_one(&literals));
            }
        }
    }
}

fn each_position_contains_exactly_one_number(solver: &mut Solver) {
    for row in 1..=PUZZLE_ISIZE {
        for column in 1..=PUZZLE_ISIZE {
            let literals: Vec<_> = (1..=PUZZLE_ISIZE)
                .map(|value| Lit::from(&PositionWithValue { row, column, value }))
                .collect();
            solver.add_formula(&exactly_one(&literals));
        }
    }
}

/// Produces a formula that requires that exactly one of the input literals is true.
fn exactly_one(literals: &[Lit]) -> CnfFormula {
    let mut formula = CnfFormula::new();

    // at least one literal is true
    formula.add_clause(literals);

    // at most one literal is true
    for (index, i) in literals.iter().enumerate() {
        for j in literals.iter().skip(index + 1) {
            formula.add_clause(&[!i.clone(), !j.clone()]);
        }
    }

    formula
}

#[derive(Debug)]
pub enum PuzzleSolverError {
    SolverError(SolverError),
    NotSatisfiable,
    NoModel,
}

impl fmt::Display for PuzzleSolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PuzzleSolverError::SolverError(e) => e.fmt(f),
            PuzzleSolverError::NotSatisfiable => write!(f, "the problem is not satisfiable"),
            PuzzleSolverError::NoModel => write!(f, "the solver did not produce a model"),
        }
    }
}

impl error::Error for PuzzleSolverError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<SolverError> for PuzzleSolverError {
    fn from(error: SolverError) -> Self {
        PuzzleSolverError::SolverError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Puzzle;

    #[test]
    fn it_requires_exactly_one_true_literal() {
        let (a, b, c) = (
            Lit::from_dimacs(1),
            Lit::from_dimacs(2),
            Lit::from_dimacs(3),
        );
        let mut solver = Solver::new();
        solver.add_formula(&exactly_one(&[a, b, c]));

        // the formula is satisfiable
        assert!(solver.solve().unwrap());

        // is satisfiable with one true variable
        solver.assume(&[a, !b, !c]);
        assert!(solver.solve().unwrap());

        // is not satisfiable with no true variables
        solver.assume(&[!a, !b, !c]);
        assert_eq!(solver.solve().unwrap(), false);

        // is not satisfiable with two true variables
        solver.assume(&[a, !b, c]);
        assert_eq!(solver.solve().unwrap(), false);
    }

    #[test]
    fn it_solves_a_puzzle() {
        let puzzle = r"
            ┌──┬──┬──┰──┬──┬──┰──┬──┬──┐
            │ 5│ 3│  ┃  │ 7│  ┃  │  │  │
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 6│  │  ┃ 1│ 9│ 5┃  │  │  │
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │  │ 9│ 8┃  │  │  ┃  │ 6│  │
            ┝━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┥
            │ 8│  │  ┃  │ 6│  ┃  │  │ 3│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 4│  │  ┃ 8│  │ 3┃  │  │ 1│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 7│  │  ┃  │ 2│  ┃  │  │ 6│
            ┝━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┥
            │  │ 6│  ┃  │  │  ┃ 2│ 8│  │
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │  │  │  ┃ 4│ 1│ 9┃  │  │ 5│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │  │  │  ┃  │ 8│  ┃  │ 7│ 9│
            └──┴──┴──┸──┴──┴──┸──┴──┴──┘
        "
        .parse::<Puzzle>()
        .unwrap();
        let expected = r"
            ┌──┬──┬──┰──┬──┬──┰──┬──┬──┐
            │ 5│ 3│ 4┃ 6│ 7│ 8┃ 9│ 1│ 2│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 6│ 7│ 2┃ 1│ 9│ 5┃ 3│ 4│ 8│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 1│ 9│ 8┃ 3│ 4│ 2┃ 5│ 6│ 7│
            ┝━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┥
            │ 8│ 5│ 9┃ 7│ 6│ 1┃ 4│ 2│ 3│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 4│ 2│ 6┃ 8│ 5│ 3┃ 7│ 9│ 1│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 7│ 1│ 3┃ 9│ 2│ 4┃ 8│ 5│ 6│
            ┝━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┥
            │ 9│ 6│ 1┃ 5│ 3│ 7┃ 2│ 8│ 4│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 2│ 8│ 7┃ 4│ 1│ 9┃ 6│ 3│ 5│
            ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
            │ 3│ 4│ 5┃ 2│ 8│ 6┃ 1│ 7│ 9│
            └──┴──┴──┸──┴──┴──┸──┴──┴──┘
        "
        .parse::<Puzzle>()
        .unwrap();
        let solved_puzzle = solve_puzzle(&puzzle).unwrap();
        assert_eq!(expected, solved_puzzle);
    }
}
