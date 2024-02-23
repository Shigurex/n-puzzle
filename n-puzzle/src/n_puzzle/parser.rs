use anyhow::{Result, anyhow};
use crate::n_puzzle::Pos;

use super::Puzzle;
use std::fs;

impl Puzzle {
    pub(super) fn parse_text(text_path: String) -> Result<Self> {
        let text = fs::read_to_string(text_path)?;
        let text_without_comments: String = text.lines().map(|line| {
            match line.find('#') {
                Some(index) => &line[0..index],
                _ => line
            }
        }).collect::<Vec<&str>>().join(" ");
        let mut elements: Vec<&str> = text_without_comments.split_whitespace().collect();

        let size: &str = match elements.first() {
            Some(&elem) => elem,
            None => return Err(anyhow!("Cannot find the size.")),
        };
        elements.remove(0);
        let size: usize = size.parse()?;
        let mut state = vec![vec![0; size]; size];
        let mut blank_pos = Pos::new(0, 0);

        if elements.len() != size * size {
            return Err(anyhow!("Number of elements does not match puzzle size: {}.", size))
        }
        for (index, element) in elements.iter().enumerate() {
            let val = element.parse::<usize>()?;
            let pos = Pos::new(index % size, index / size);
            state[pos.y][pos.x] = val;
            if state[pos.y][pos.x] == 0 {
                blank_pos.x = pos.x;
                blank_pos.y = pos.y;
            }
        }

        let puzzle = Self { size, state, blank_pos };
        if !puzzle.check_state() {
            return Err(anyhow!("Invalid puzzle format."))
        }
        Ok(puzzle)
    }
}
