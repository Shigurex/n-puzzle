use anyhow::Result;
use super::Puzzle;

impl Puzzle {
    pub(super) fn generate(size: usize) -> Result<Self> {
        Ok(Puzzle::new_answer(size))
    }
}
