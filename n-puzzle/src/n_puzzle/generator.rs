use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;
use super::Puzzle;
use super::pos::Pos;

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
        Ok(Self { size, state, blank_pos })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_generate_border() {
        let puzzle = Puzzle::generate(2).unwrap();
        let result = puzzle.check_state();
        assert_eq!(result, true);
    }

    #[test]
    fn test_generate_normal() {
        let puzzle = Puzzle::generate(5).unwrap();
        let result = puzzle.check_state();
        assert_eq!(result, true);
    }
}
