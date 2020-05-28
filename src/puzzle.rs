use crate::constants::{PUZZLE_ISIZE, PUZZLE_USIZE};
use crate::position::PositionWithValue;
use regex::Regex;
use std::convert::TryInto;
use std::error;
use std::fmt;
use std::str::FromStr;
use varisat::Lit;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Puzzle {
    values: Vec<Option<isize>>,
}

impl Puzzle {
    pub fn from_model(model: &[Lit]) -> Puzzle {
        let mut values: Vec<Option<isize>> =
            Vec::with_capacity((PUZZLE_ISIZE * PUZZLE_ISIZE).try_into().unwrap());
        for row in 1..=PUZZLE_ISIZE {
            for column in 1..=PUZZLE_ISIZE {
                let value = (1..=PUZZLE_ISIZE).fold(None, |_, v| {
                    let lit = Lit::from(&PositionWithValue {
                        row,
                        column,
                        value: v,
                    });
                    if model.contains(&lit) {
                        Some(v)
                    } else {
                        None
                    }
                });
                values.push(value);
            }
        }
        Puzzle { values }
    }

    pub fn to_model(&self) -> Vec<Lit> {
        let mut lits = Vec::new();
        for row in 0..PUZZLE_USIZE {
            for column in 0..PUZZLE_USIZE {
                match self.values[row * PUZZLE_USIZE + column] {
                    Some(value) => {
                        for v in 1..=PUZZLE_ISIZE {
                            let lit = Lit::from(&PositionWithValue {
                                row: (row + 1) as isize,
                                column: (column + 1) as isize,
                                value: v,
                            });
                            if v == value {
                                lits.push(lit);
                            } else {
                                lits.push(!lit);
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        lits
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "┌──┬──┬──┰──┬──┬──┰──┬──┬──┐\n")?;
        for row in 0..PUZZLE_USIZE {
            write!(f, "│")?;
            for column in 0..PUZZLE_USIZE {
                write!(f, " ")?; // leading space
                match self.values[row * PUZZLE_USIZE + column] {
                    Some(value) => write!(f, "{}", value.to_string())?,
                    None => write!(f, " ")?,
                };
                if column == 2 || column == 5 {
                    write!(f, "┃")?;
                } else {
                    write!(f, "│")?;
                }
            }
            if row == 2 || row == 5 {
                write!(f, "\n┝━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┥\n")?;
            } else if row < (PUZZLE_USIZE - 1) {
                write!(f, "\n├──┼──┼──╂──┼──┼──╂──┼──┼──┤\n")?;
            } else {
                write!(f, "\n└──┴──┴──┸──┴──┴──┸──┴──┴──┘\n")?;
            }
        }
        Ok(())
    }
}

impl FromStr for Puzzle {
    type Err = InvalidSizeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<Option<isize>> =
            Vec::with_capacity((PUZZLE_ISIZE * PUZZLE_ISIZE).try_into().unwrap());

        let cell_delimiter_pattern = Regex::new(r"[^\d\s]+").unwrap();

        let cell_lines = input
            .trim()
            .lines()
            .map(|line| cell_delimiter_pattern.replace_all(line.trim(), "|"))
            .filter(|line| line.len() > PUZZLE_USIZE)
            .take(PUZZLE_USIZE);

        for line in cell_lines {
            for cell_value in line.split("|").skip(1).take(PUZZLE_USIZE) {
                match cell_value.trim().parse::<isize>() {
                    Ok(value) => values.push(Some(value)),
                    Err(_) => values.push(None),
                }
            }
        }

        if values.len() == PUZZLE_USIZE * PUZZLE_USIZE {
            Ok(Puzzle { values })
        } else {
            Err(InvalidSizeError {
                actual_size: values.len(),
            })
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InvalidSizeError {
    actual_size: usize,
}

impl fmt::Display for InvalidSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid puzzle size: {}", self.actual_size)
    }
}

impl error::Error for InvalidSizeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_and_draws_a_puzzle() {
        let input = r"
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
        ";
        let leading_whitespace = Regex::new(r"(?m:^\s*)").unwrap();
        assert_eq!(
            input.parse::<Puzzle>().unwrap().to_string().trim(),
            leading_whitespace.replace_all(input.trim(), "")
        );
    }

    #[test]
    fn it_produces_a_model_representing_filled_cells() {
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
        let model_roundtrip = Puzzle::from_model(&puzzle.to_model());
        assert_eq!(model_roundtrip, puzzle);
    }
}
