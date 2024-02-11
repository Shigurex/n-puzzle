mod args;
mod n_puzzle;
mod algorithm;

pub use n_puzzle::{PuzzleSettings, Puzzle};
pub use algorithm::{Algorithm, Heuristic};

use anyhow::Result;
use args::parse_args;
use algorithm::Solver;

fn run() -> Result<()> {
    // Parse arguments
    let settings = parse_args()?;
    // Generate puzzle
    let puzzle = Puzzle::new(settings.puzzle_settings)?;
    // Solve puzzle
    let solver = Solver::new(settings.algorithm, settings.heuristic, puzzle);
    solver.solve()?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
