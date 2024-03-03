mod algorithm;
mod args;
mod n_puzzle;

pub use algorithm::{Algorithm, Heuristic};
pub use n_puzzle::{Move, Puzzle, PuzzleSettings};

use algorithm::Solver;
use anyhow::Result;
use args::{get_args, parse_args};

fn run() -> Result<()> {
    // Parse arguments
    let args = get_args();
    let settings = match parse_args(args) {
        Ok(Some(settings)) => settings,
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
    // Generate puzzle
    let puzzle = Puzzle::new(settings.puzzle_settings)?;
    // Solve puzzle
    let solver = Solver::new(
        settings.algorithm.unwrap(),
        settings.heuristic,
        puzzle,
        settings.timeout,
    );
    solver.solve(settings.verbose)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
