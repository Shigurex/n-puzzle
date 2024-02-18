mod args;
mod n_puzzle;
mod algorithm;

pub use n_puzzle::{PuzzleSettings, Puzzle, Move};
pub use algorithm::{Algorithm, Heuristic};

use anyhow::Result;
use args::{get_args, parse_args};
use algorithm::Solver;

fn run() -> Result<()> {
    // Parse arguments
    let args = get_args();
    let settings = match parse_args(args) {
        Ok(Some(settings)) => settings,
        Ok(_) => return Ok(()),
        Err(e) => return Err(e)
    };
    // Generate puzzle
    let puzzle = Puzzle::new(settings.puzzle_settings)?;
    // Solve puzzle
    let solver = Solver::new(settings.algorithm.unwrap(), settings.heuristic, puzzle);
    solver.solve(true)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
