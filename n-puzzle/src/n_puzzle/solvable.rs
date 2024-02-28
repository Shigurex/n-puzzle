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
        let blank_pos = self.blank_pos;
        let pos = blank_pos.y * self.size + blank_pos.x;
        flat_state[pos] = self.size * self.size;
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
        let (x, y) = (self.size - 1 - blank_pos.x, self.size - 1 - blank_pos.y);
        Ok((count + x + y) % 2 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_trivial_solvable() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]])?;
        assert!(puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_is_not_trivial_solvable() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![4, 5, 6], vec![8, 7, 0]])?;
        assert!(!puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_is_solvable_4x4() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![
            vec![6, 5, 3, 8],
            vec![2, 1, 7, 4],
            vec![13, 0, 11, 15],
            vec![9, 14, 10, 12],
        ])?;
        assert!(puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_is_not_solvable_4x4() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![
            vec![6, 5, 3, 8],
            vec![2, 1, 7, 4],
            vec![13, 0, 11, 15],
            vec![9, 14, 12, 10],
        ])?;
        assert!(!puzzle.is_solvable()?);
        Ok(())
    }

    #[test]
    fn test_unsolvable_case() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![3, 4, 6], vec![7, 1, 0], vec![2, 8, 5]])?;
        assert!(!puzzle.is_solvable()?);
        Ok(())
    }
}
