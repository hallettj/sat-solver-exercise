use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula, Lit};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct PuzzleConfig {
    size: isize,
}

/// One position in a Sudoku puzzle with the value in that position. Row and column values start
/// from 1.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    row: isize,
    column: isize,
    value: isize,
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
fn lit_for_position(config: &PuzzleConfig, pos: &Position) -> Lit {
    let s = config.size;
    let linear_coord = ((pos.row - 1) * s * s) + ((pos.column - 1) * s) + (pos.value - 1);
    // dimacs variable numbers start from 1
    Lit::from_dimacs(linear_coord + 1)
}

fn main() {
    let mut solver = Solver::new();
    // let (a, b, c) = solver.new_lits();
    let (a, b, c) = (
        Lit::from_dimacs(1),
        Lit::from_dimacs(2),
        Lit::from_dimacs(3),
    );

    solver.add_clause(&[a, b, c]);
    solver.add_clause(&[!a, !b]);
    solver.add_clause(&[!a, !c]);

    solver.assume(&[Lit::from_dimacs(-1)]);

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
        let config = PuzzleConfig { size: 9 };

        let literals: Vec<Lit> = (1..=config.size).into_iter().flat_map(move |r|
            (1..=config.size).into_iter().flat_map(move |c|
                (1..=config.size).into_iter().map(move |v|
                    lit_for_position(&config, &Position { row: r, column: c, value: v})
                )
            )
        ).collect();

        for (index, lit) in literals.iter().enumerate() {
            if index == 0 {
                assert_eq!(lit.to_dimacs(), 1);
            } else {
                assert_eq!(lit.to_dimacs(), literals[index - 1].to_dimacs() + 1);
            }
        }
    }
}
