use crate::constants::PUZZLE_ISIZE;
use std::ops::RangeInclusive;

/// Returns an iterator that produces each combination of two puzzle dimensions.
/// That could be rows and columns, columns and values, or rows and values.
/// In a 9x9 Sudoku puzzle this results in 9^2 calls.
pub fn over_2_dimensions() -> impl Iterator<Item = (isize, isize)> {
    (1..=PUZZLE_ISIZE).flat_map(move |x| (1..=PUZZLE_ISIZE).map(move |y| (x, y)))
}

/// Returns an iterator that produces each possible value for a puzzle cell.
pub fn values() -> RangeInclusive<isize> {
    1..=PUZZLE_ISIZE
}
