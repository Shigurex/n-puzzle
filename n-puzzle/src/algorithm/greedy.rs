use super::Output;
use crate::Puzzle;
use anyhow::Result;

pub(super) fn solve(_puzzle: &Puzzle, _timeout: Option<u64>) -> Result<Output> {
    Ok(Output::new(0, 0, vec![]))
}
