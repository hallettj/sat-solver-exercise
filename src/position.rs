use crate::constants::PUZZLE_ISIZE;
use varisat::Lit;

/// One position in a Sudoku puzzle with the value in that position. Row and column values start
/// from 1.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PositionWithValue {
    pub row: isize,
    pub column: isize,
    pub value: isize,
}

/// A Sudoku puzzle is a table of numbers. But SAT variables can only be true or false, so We
/// represent a puzzle as a 3D table with one coordinate for rows, another for columns, and a third
/// for each of the possible values in a single row-column cell.
///
/// A SAT variable is represented by one number. This function maps each position in the 3D table
/// to a uniquely-numbered literal.
impl From<&PositionWithValue> for Lit {
    fn from(pos: &PositionWithValue) -> Self {
        let s = PUZZLE_ISIZE;
        let linear_coord = ((pos.row - 1) * s * s) + ((pos.column - 1) * s) + (pos.value - 1);
        // dimacs variable numbers start from 1
        Lit::from_dimacs(linear_coord + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_assigns_unique_literal_to_each_puzzle_position() {
        let literals: Vec<Lit> = (1..=PUZZLE_ISIZE)
            .flat_map(move |r| {
                (1..=PUZZLE_ISIZE).flat_map(move |c| {
                    (1..=PUZZLE_ISIZE).map(move |v| {
                        Lit::from(&PositionWithValue {
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
}
