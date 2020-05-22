use crate::constants::PUZZLE_ISIZE;
use crate::position::PositionWithValue;
use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula, Lit};

pub fn sudoku(solver: &mut Solver) {
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
                                row: row + box_row,
                                column: column + box_column,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
