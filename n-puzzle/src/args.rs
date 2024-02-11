use anyhow::Result;
use super::{
    PuzzleSetings, Algorithm, Heuristic,
};

pub struct Settings {
    pub puzzle_settings: PuzzleSetings,
    pub algorithm: Algorithm,
    pub heuristic: Heuristic,
}

impl Settings {
    pub fn new(puzzle_settings: PuzzleSetings, algorithm: Algorithm, heuristic: Heuristic) -> Self {
        Self {
            puzzle_settings,
            algorithm,
            heuristic,
        }
    }

}

pub fn parse_args() -> Result<Settings> {
    // Parse arguments
    Ok(Settings::new(
        PuzzleSetings::Size(3),
        Algorithm::AStar,
        Heuristic::Manhattan
    ))
}
