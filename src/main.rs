mod constants;
mod iterate;
mod position;
pub mod puzzle;
pub mod solver;

use crate::puzzle::Puzzle;
use crate::solver::solve_puzzle;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    match read_and_solve_puzzle() {
        Ok(_) => Ok(()),
        Err(error) => {
            eprintln!("Error: {}", error);
            Ok(())
        }
    }
}

fn read_and_solve_puzzle() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let puzzle = buffer
        .parse::<Puzzle>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let solution =
        solve_puzzle(&puzzle).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    println!("{}", &solution);
    Ok(())
}
