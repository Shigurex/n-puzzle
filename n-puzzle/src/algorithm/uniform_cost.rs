use super::{astar, Output};
use crate::Puzzle;
use anyhow::Result;

fn uniform_cost(_puzzle: &Puzzle) -> usize {
    0
}

pub(super) fn solve(puzzle: &Puzzle) -> Result<Output> {
    astar(puzzle.clone(), uniform_cost, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_cost() {
        let mut puzzle = Puzzle::generate_solvable(3).unwrap();
        let output = astar(puzzle.clone(), uniform_cost, false).unwrap();
        for m in output.path {
            puzzle.move_blank(m).unwrap();
        }
        assert!(puzzle.is_final_state());
    }
}
