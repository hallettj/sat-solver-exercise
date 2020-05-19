// use std::convert::TryInto;
use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula, Lit};

const PUZZLE_SIZE: isize = 9;

/// One position in a Sudoku puzzle with the value in that position. Row and column values start
/// from 1.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct PositionWithValue {
    row: isize,
    column: isize,
    value: isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    row: isize,
    column: isize,
    value: Option<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Puzzle {
    positions: Vec<Position>,
}

// impl Puzzle {
//     fn from_model(model: &[Lit]) -> Puzzle {
//         let mut positions: Vec<Position> =
//             Vec::with_capacity((PUZZLE_SIZE * PUZZLE_SIZE).try_into().unwrap());

//         let positions: Vec<_> = (1..=PUZZLE_SIZE)
//             .flat_map(|row| {
//                 (1..=PUZZLE_SIZE).map(|column| {
//                     let value = (1..=PUZZLE_SIZE).fold(None, |_, v| {
//                         let lit = lit_for_position(&PositionWithValue {
//                             row,
//                             column,
//                             value: v,
//                         });
//                         if model.contains(&lit) {
//                             Some(v)
//                         } else {
//                             None
//                         }
//                     });
//                     Position { row, column, value }
//                 })
//             })
//             .collect();
//         Puzzle { positions }
//     }
// }

fn sudoku(solver: &mut Solver) {
    no_row_contains_duplicate_numbers(solver);
    no_column_contains_duplicate_numbers(solver);
    no_3x3_boxes_contain_duplicate_numbers(solver);
    each_position_contains_exactly_one_number(solver);
}

fn no_row_contains_duplicate_numbers(solver: &mut Solver) {
    for row in 1..=PUZZLE_SIZE {
        for value in 1..=PUZZLE_SIZE {
            let literals: Vec<_> = (1..=PUZZLE_SIZE)
                .map(|column| lit_for_position(&PositionWithValue { row, column, value }))
                .collect();
            solver.add_formula(&exactly_one(&literals))
        }
    }
}

fn no_column_contains_duplicate_numbers(solver: &mut Solver) {
    for column in 1..=PUZZLE_SIZE {
        for value in 1..=PUZZLE_SIZE {
            let literals: Vec<_> = (1..=PUZZLE_SIZE)
                .map(|row| lit_for_position(&PositionWithValue { row, column, value }))
                .collect();
            solver.add_formula(&exactly_one(&literals))
        }
    }
}

fn no_3x3_boxes_contain_duplicate_numbers(solver: &mut Solver) {
    for box_row in 0..3 {
        for box_column in 0..3 {
            for value in 1..=PUZZLE_SIZE {
                let literals: Vec<_> = (1..=3)
                    .flat_map(move |row| {
                        (1..=3).map(move |column| {
                            lit_for_position(&PositionWithValue {
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
    for row in 1..=PUZZLE_SIZE {
        for column in 1..=PUZZLE_SIZE {
            let literals: Vec<_> = (1..=PUZZLE_SIZE)
                .map(|value| lit_for_position(&PositionWithValue { row, column, value }))
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

/// A Sudoku puzzle is a table of numbers. But SAT variables can only be true or false, so We
/// represent a puzzle as a 3D table with one coordinate for rows, another for columns, and a third
/// for each of the possible values in a single row-column cell.
///
/// A SAT variable is represented by one number. This function maps each position in the 3D table
/// to a uniquely-numbered literal.
fn lit_for_position(pos: &PositionWithValue) -> Lit {
    let s = PUZZLE_SIZE;
    let linear_coord = ((pos.row - 1) * s * s) + ((pos.column - 1) * s) + (pos.value - 1);
    // dimacs variable numbers start from 1
    Lit::from_dimacs(linear_coord + 1)
}

fn position_for_lit(lit: &Lit) -> Position {
    let mut n = lit.to_dimacs() - 1;
    let value = (n % PUZZLE_SIZE) + 1;
    n /= PUZZLE_SIZE;
    let column = (n % PUZZLE_SIZE) + 1;
    n /= PUZZLE_SIZE;
    let row = n + 1;
    Position {
        row,
        column,
        value: if lit.is_positive() { Some(value) } else { None },
    }
}

// fn draw_puzzle(literals: Vec<Lit>) -> String {

// }

fn main() {
    let mut solver = Solver::new();

    sudoku(&mut solver);

    let solution = solver.solve().unwrap();
    let model = solver.model();
    println!("solution: {:?}, model: {:?}", solution, model);
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

    #[test]
    fn it_assigns_unique_literal_to_each_puzzle_position() {
        let literals: Vec<Lit> = (1..=PUZZLE_SIZE)
            .flat_map(move |r| {
                (1..=PUZZLE_SIZE).flat_map(move |c| {
                    (1..=PUZZLE_SIZE).map(move |v| {
                        lit_for_position(&PositionWithValue {
                            row: r,
                            column: c,
                            value: v,
                        })
                    })
                })
            })
            .collect();

        for (index, lit) in literals.iter().enumerate() {
            if index == 0 {
                assert_eq!(lit.to_dimacs(), 1);
            } else {
                assert_eq!(lit.to_dimacs(), literals[index - 1].to_dimacs() + 1);
            }
        }
    }

    #[test]
    fn it_preserves_position_with_value_on_roundtrip_through_lit() {
        for row in 1..=PUZZLE_SIZE {
            for column in 1..=PUZZLE_SIZE {
                for value in 1..=PUZZLE_SIZE {
                    let input = PositionWithValue { row, column, value };
                    let expected = Position {
                        row,
                        column,
                        value: Some(value),
                    };
                    assert_eq!(position_for_lit(&lit_for_position(&input)), expected);
                }
            }
        }
    }
}
