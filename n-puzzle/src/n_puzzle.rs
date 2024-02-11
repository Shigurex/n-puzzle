mod parser;
mod generator;

use anyhow::Result;

pub enum PuzzleSettings {
    Size(usize),
    TextPath(String),
}

pub struct Puzzle {}

impl Puzzle {
    pub fn new(settings: PuzzleSettings) -> Result<Self> {
        match settings {
            PuzzleSettings::Size(size) => Self::generate(size),
            PuzzleSettings::TextPath(text_path) => Self::parse_text(text_path),
        }
    }
}
