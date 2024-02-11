use anyhow::Result;
use super::{
    PuzzleSettings, Algorithm, Heuristic,
};

pub struct Settings {
    pub puzzle_settings: PuzzleSettings,
    pub algorithm: Algorithm,
    pub heuristic: Heuristic,
}

impl Settings {
    pub fn new(puzzle_settings: PuzzleSettings, algorithm: Algorithm, heuristic: Heuristic) -> Self {
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
        PuzzleSettings::Size(3),
        Algorithm::AStar,
        Heuristic::Manhattan
    ))
}
