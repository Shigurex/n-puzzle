use super::Puzzle;
use anyhow::{anyhow, Result};

impl Puzzle {
    /// Check if the puzzle is solvable.
    ///
    /// # Reference
    ///
    /// - https://manabitimes.jp/math/979
    /// - https://edu-gw2.math.cst.nihon-u.ac.jp/~kurino/2006/linear/permutation/permutation.pdf
    pub(super) fn is_solvable(&self) -> Result<bool> {
        let mut count = 0;
        let mut flat_state: Vec<usize> = self.state.iter().flatten().copied().collect();
        let answer_map = Puzzle::generate_arrange_order_answer_map(self.size, false);
        flat_state = flat_state
            .iter()
            .map(|&x| answer_map.get(&x).unwrap())
            .copied()
            .collect();
        for i in 0..flat_state.len() {
            if flat_state[i] == i + 1 {
                continue;
            }
            let index = flat_state
                .iter()
                .skip(i)
                .position(|&x| x == i + 1)
                .ok_or_else(|| anyhow!("Cannot find the element."))?;
            flat_state.swap(i, index + i);
            count += 1;
        }
        let blank_pos = self.blank_pos;
        let answer = Puzzle::new_answer(self.size);
        let blank_pos_diff = (blank_pos.x as isize - answer.blank_pos.x as isize).abs()
            + (blank_pos.y as isize - answer.blank_pos.y as isize).abs();
        Ok((count + blank_pos_diff) % 2 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_trivial_solvable() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]])?;
        assert!(puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_is_not_trivial_solvable() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![8, 0, 7], vec![4, 6, 5]])?;
        assert!(!puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_is_solvable_4x4() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![
            vec![8, 1, 7, 3],
            vec![5, 2, 6, 12],
            vec![11, 0, 4, 14],
            vec![10, 13, 15, 9],
        ])?;
        assert!(puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_is_not_solvable_4x4() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![
            vec![8, 1, 7, 3],
            vec![5, 2, 6, 12],
            vec![11, 0, 4, 14],
            vec![10, 13, 9, 15],
        ])?;
        assert!(!puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_unsolvable_case() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![5, 2, 4], vec![8, 1, 6], vec![0, 3, 7]])?;
        assert!(!puzzle.is_solvable()?);
        Ok(())
    }
}
