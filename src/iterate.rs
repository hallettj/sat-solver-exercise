use crate::constants::PUZZLE_ISIZE;
use std::ops::RangeInclusive;

/// Returns an iterator that produces each combination of row and column.
/// In a 9x9 Sudoku puzzle this results in 9^2 calls.
pub fn cells() -> impl Iterator<Item = (isize, isize)> {
    (1..=PUZZLE_ISIZE).flat_map(move |row| (1..=PUZZLE_ISIZE).map(move |column| (row, column)))
}

/// Returns an iterator that produces each possible value for a puzzle cell.
pub fn values() -> RangeInclusive<isize> {
    1..=PUZZLE_ISIZE
}

