use anyhow::Result;
use crate::Puzzle;
use super::{Heuristic, Output};

pub(super) fn solve(_puzzle: &Puzzle, _heuristic: Heuristic) -> Result<Output> {
    Ok(Output::new(0, 0, vec![]))
}
