use super::{astar, Heuristic, Output};
use crate::Puzzle;
use anyhow::Result;

pub(super) fn solve(puzzle: &Puzzle, heuristic: Heuristic, timeout: Option<u64>) -> Result<Output> {
    astar(
        puzzle.clone(),
        Heuristic::get_heuristic(&heuristic)?,
        true,
        timeout,
    )
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_greedy_trivial() -> Result<()> {
    //     let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 0, 8]])?;
    //     let output = astar(puzzle.clone(), |_| 0, true)?;
    //     assert!(output.path.len() == 1);
    //     assert!(output.path[0] == crate::Move::Right);
    //     Ok(())
    // }
}
