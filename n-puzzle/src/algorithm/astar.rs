use super::{Heuristic, Output};
use crate::Puzzle;
use anyhow::Result;

pub(super) fn solve(_puzzle: &Puzzle, _heuristic: Heuristic) -> Result<Output> {
    Ok(Output::new(0, 0, vec![]))
}
