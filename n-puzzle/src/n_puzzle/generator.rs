use super::{Pos, Puzzle};
use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;

impl Puzzle {
    pub(super) fn generate(size: usize) -> Result<Self> {
        if size <= 1 {
            return Err(anyhow!("invalid size selected"));
        }

        let mut state = vec![vec![0; size]; size];
        let mut blank_pos = Pos::new(0, 0);
        let mut rand_state: Vec<usize> = (0..=size * size - 1).collect();
        let mut rng = rand::thread_rng();
        rand_state.shuffle(&mut rng);

        for i in 0..size {
            for j in 0..size {
                state[i][j] = rand_state[i * size + j];
                if state[i][j] == 0 {
                    blank_pos.x = j;
                    blank_pos.y = i;
                }
            }
        }
        Ok(Self {
            size,
            state,
            blank_pos,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_border() {
        let puzzle = Puzzle::generate(2).unwrap();
        assert!(puzzle.check_state());
    }

    #[test]
    fn test_generate_normal() {
        let puzzle = Puzzle::generate(5).unwrap();
        let result = puzzle.check_state();
        assert!(result);
    }

    #[test]
    fn test_generate_invalid() {
        let puzzle = Puzzle::generate(1);
        assert!(puzzle.is_err());
    }
}
