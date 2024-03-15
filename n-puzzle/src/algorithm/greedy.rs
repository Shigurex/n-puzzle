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
    use super::*;

    #[test]
    fn test_greedy_trivial() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![0, 8, 4], vec![7, 6, 5]])?;
        let output = solve(&puzzle, Heuristic::Hamming, None)?;
        assert!(output.path.len() == 1);
        assert!(output.path[0] == crate::Move::Right);
        Ok(())
    }

    #[test]
    fn test_greedy() -> Result<()> {
        let mut puzzle = Puzzle::new_from_state(vec![vec![0, 2, 3], vec![1, 8, 4], vec![7, 6, 5]])?;
        let output = solve(&puzzle, Heuristic::Hamming, None)?;
        assert!(output.path.len() == 2);
        for m in output.path {
            puzzle.move_blank(m).unwrap();
        }
        assert!(puzzle.is_final_state());
        Ok(())
    }

    #[test]
    fn test_greedy_unsolvable() {
        let puzzle =
            Puzzle::new_from_state(vec![vec![1, 0, 6], vec![5, 3, 8], vec![4, 2, 7]]).unwrap();
        assert!(solve(&puzzle, Heuristic::Hamming, None).is_err())
    }
}
