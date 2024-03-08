mod algorithm;
mod args;
mod n_puzzle;

pub use algorithm::{Algorithm, Heuristic};
pub use n_puzzle::{Move, Pos, Puzzle, PuzzleSettings};

use algorithm::Solver;
use anyhow::Result;
use args::{get_args, parse_args};

const MAX_PUZZLE_SIZE: usize = 100;

pub fn run(args: Vec<String>) -> Result<()> {
    // Parse arguments
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

pub fn cui_run() -> Result<()> {
    let args = get_args();
    run(args)
}
