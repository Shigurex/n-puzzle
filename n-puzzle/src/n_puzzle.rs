mod parser;
mod generator;

use anyhow::Result;

pub enum PuzzleSetings {
    Size(usize),
    TextPath(String),
}

pub struct Puzzle {}

impl Puzzle {
    pub fn new(settings: PuzzleSetings) -> Result<Self> {
        match settings {
            PuzzleSetings::Size(size) => Self::generate(size),
            PuzzleSetings::TextPath(text_path) => Self::parse_text(text_path),
        }
    }
}
