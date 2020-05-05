use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula, Lit};

/// Produces a formula that requires that exactly one of the input literals is true.
fn exactly_one(literals: &[Lit]) -> CnfFormula {
    let mut formula = CnfFormula::new();

    // at least is true
    formula.add_clause(literals);

    // at most one is true
    for (index, i) in literals.iter().enumerate() {
        for j in literals.iter().skip(index + 1) {
            formula.add_clause(&[!i.clone(), !j.clone()]);
        }
    }

    formula
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
}
