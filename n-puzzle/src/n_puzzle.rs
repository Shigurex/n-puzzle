mod parser;
mod generator;

use anyhow::{Result, anyhow};

pub enum PuzzleSettings {
    Size(usize),
    TextPath(String),
}

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    size: usize,
    state: Vec<Vec<usize>>,
    blank_pos: (usize, usize),
}

impl Puzzle {
    /// Generate a new puzzle with PuzzleSettings
    pub fn new(settings: PuzzleSettings) -> Result<Self> {
        match settings {
            PuzzleSettings::Size(size) => Self::generate(size),
            PuzzleSettings::TextPath(text_path) => Self::parse_text(text_path),
        }
    }

    /// Generate a answer puzzle with the given size
    pub fn new_answer(size: usize) -> Self {
        let mut state = vec![vec![0; size]; size];
        let mut count = 1;
        for i in 0..size {
            for j in 0..size {
                state[i][j] = count;
                count += 1;
            }
        }
        state[size - 1][size - 1] = 0;
        Self { size, state, blank_pos: (size - 1, size - 1) }
    }

    /// Check puzzle state
    pub fn check_state(&self) -> bool {
        let mut state = vec![false; self.size * self.size];
        if self.state.len() != self.size {
            return false;
        }
        for row in &self.state {
            if row.len() != self.size {
                return false
            }
            for val in row {
                if *val >= self.size * self.size {
                    return false;
                }
                if state[*val] {
                    return false;
                }
                state[*val] = true;
            }
        }
        // Check blank position
        let val = match self.get(self.blank_pos.0, self.blank_pos.1) {
            Ok(val) => val,
            Err(_) => return false,
        };
        if val != 0 {
            return false;
        }
        return true
    }

    /// Checl if the puzzle is in the final state
    pub fn is_final_state(&self) -> bool {
        let mut count = 1;
        for i in 0..self.size {
            for j in 0..self.size {
                if count == self.size * self.size {
                    return true;
                }
                if self.state[i][j] != count {
                    return false;
                }
                count += 1;
            }
        }
        false
    }

    /// Get the value at the given position
    pub fn get(&self, i: usize, j: usize) -> Result<usize> {
        if i >= self.size || j >= self.size {
            return Err(anyhow!("Index out of bounds: ({}, {})", i, j));
        }
        Ok(self.state[i][j])
    }

    /// Set the value at the given position without value checking
    pub fn unchecked_set(&mut self, i: usize, j: usize, val: usize) -> Result<()> {
        if i >= self.size || j >= self.size {
            return Err(anyhow!("Index out of bounds: ({}, {})", i, j));
        }
        self.state[i][j] = val;
        if val == 0 {
            self.blank_pos = (i, j);
        }
        Ok(())
    }

    /// Set the value at the given position with value checking
    pub fn set(&mut self, i: usize, j: usize, val: usize) -> Result<()> {
        if val >= self.size * self.size {
            return Err(anyhow!("Value out of bounds: {}", val));
        }
        for row in &self.state {
            if row.contains(&val) {
                return Err(anyhow!("Value already exists: {}", val));
            }
        }
        self.unchecked_set(i, j, val)
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_blank_pos(&self) -> (usize, usize) {
        self.blank_pos
    }

    /// Swap the values at the given positions
    pub fn swap(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) -> Result<()> {
        if i1 >= self.size || j1 >= self.size || i2 >= self.size || j2 >= self.size {
            return Err(anyhow!("Index out of bounds: ({}, {}), ({}, {})", i1, j1, i2, j2));
        }
        let val1 = self.get(i1, j1)?;
        let val2 = self.get(i2, j2)?;
        self.state[i1][j1] = val2;
        self.state[i2][j2] = val1;
        if val1 == 0 {
            self.blank_pos = (i2, j2);
        } else if val2 == 0 {
            self.blank_pos = (i1, j1);
        }
        Ok(())
    }

    /// Move the blank position
    pub fn move_blank(&mut self, mv: Move) -> Result<()> {
        let (i, j) = self.blank_pos;
        match mv {
            Move::Up => {
                if i == 0 {
                    return Err(anyhow!("Cannot move up"));
                }
                self.swap(i, j, i - 1, j)?;
            },
            Move::Down => {
                if i == self.size - 1 {
                    return Err(anyhow!("Cannot move down"));
                }
                self.swap(i, j, i + 1, j)?;
            },
            Move::Left => {
                if j == 0 {
                    return Err(anyhow!("Cannot move left"));
                }
                self.swap(i, j, i, j - 1)?;
            },
            Move::Right => {
                if j == self.size - 1 {
                    return Err(anyhow!("Cannot move right"));
                }
                self.swap(i, j, i, j + 1)?;
            },
        }
        Ok(())
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.state {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_answer() {
        let puzzle = Puzzle::new_answer(3);
        assert_eq!(puzzle.size, 3);
        assert_eq!(puzzle.state, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]]);
    }

    #[test]
    fn test_check_state() {
        let mut puzzle = Puzzle::new_answer(3);
        assert_eq!(puzzle.check_state(), true);
        // Case where a value is duplicated
        puzzle.state[0][0] = 0;
        assert_eq!(puzzle.check_state(), false);
        puzzle.state[0][0] = 1;
        // Case where the line length is not equal to the size
        puzzle.state.pop();
        puzzle.state.push(vec![8, 9]);
        assert_eq!(puzzle.check_state(), false);
        puzzle.state.pop();
        puzzle.state.push(vec![8, 9, 0]);
        // Case where the column length is not equal to the size
        puzzle.state.push(vec![0; 3]);
        assert_eq!(puzzle.check_state(), false);
        puzzle.state.pop();
        // Case where a value is greater than the size
        puzzle.state[0][0] = 9;
        assert_eq!(puzzle.check_state(), false);
        puzzle.state[0][0] = 1;
        // Case where the blank position is not correct
        puzzle.blank_pos = (0, 0);
        assert_eq!(puzzle.check_state(), false);
    }

    #[test]
    fn test_is_final_state() {
        let mut puzzle = Puzzle::new_answer(3);
        assert_eq!(puzzle.is_final_state(), true);
        puzzle.state[0][0] = 0;
        assert_eq!(puzzle.is_final_state(), false);
    }

    #[test]
    fn test_get() {
        let puzzle = Puzzle::new_answer(3);
        assert_eq!(puzzle.get(0, 0).unwrap(), 1);
        assert_eq!(puzzle.get(1, 1).unwrap(), 5);
        assert_eq!(puzzle.get(2, 2).unwrap(), 0);
        assert!(puzzle.get(3, 0).is_err());
        assert!(puzzle.get(0, 3).is_err());
    }

    #[test]
    fn test_unchecked_set() {
        let mut puzzle = Puzzle::new_answer(3);
        puzzle.unchecked_set(0, 0, 9).unwrap();
        assert_eq!(puzzle.state[0][0], 9);
        puzzle.unchecked_set(2, 2, 1).unwrap();
        assert_eq!(puzzle.state[2][2], 1);
        puzzle.unchecked_set(2, 0, 0).unwrap();
        assert_eq!(puzzle.state[2][0], 0);
        assert_eq!(puzzle.blank_pos, (2, 0));
        assert!(puzzle.unchecked_set(3, 0, 1).is_err());
        assert!(puzzle.unchecked_set(0, 3, 1).is_err());
    }

    #[test]
    fn test_set() {
        let mut puzzle = Puzzle {
            size: 3,
            state: vec![vec![0; 3]; 3],
            blank_pos: (2, 2),
        };
        puzzle.set(0, 0, 1).unwrap();
        assert_eq!(puzzle.state[0][0], 1);
        puzzle.set(2, 2, 2).unwrap();
        assert_eq!(puzzle.state[2][2], 2);
        assert!(puzzle.set(0, 0, 2).is_err());
        assert!(puzzle.set(0, 0, 10).is_err());
        assert!(puzzle.set(3, 0, 3).is_err());
        assert!(puzzle.set(0, 3, 4).is_err());
    }

    #[test]
    fn test_swap() {
        let mut puzzle = Puzzle::new_answer(3);
        puzzle.swap(0, 0, 2, 2).unwrap();
        assert_eq!(puzzle.state[0][0], 0);
        assert_eq!(puzzle.state[2][2], 1);
        assert_eq!(puzzle.blank_pos, (0, 0));
        puzzle.swap(0, 0, 2, 2).unwrap();
        assert_eq!(puzzle.state[0][0], 1);
        assert_eq!(puzzle.state[2][2], 0);
        assert_eq!(puzzle.blank_pos, (2, 2));
        assert!(puzzle.swap(3, 0, 0, 0).is_err());
        assert!(puzzle.swap(0, 0, 0, 3).is_err());
    }

    #[test]
    fn test_move_blank() {
        let mut puzzle = Puzzle::new_answer(3);
        assert!(puzzle.move_blank(Move::Down).is_err());
        assert!(puzzle.move_blank(Move::Right).is_err());
        puzzle.move_blank(Move::Up).unwrap();
        assert_eq!(puzzle.state, vec![vec![1, 2, 3], vec![4, 5, 0], vec![7, 8, 6]]);
        puzzle.move_blank(Move::Up).unwrap();
        assert_eq!(puzzle.state, vec![vec![1, 2, 0], vec![4, 5, 3], vec![7, 8, 6]]);
        assert!(puzzle.move_blank(Move::Up).is_err());
        puzzle.move_blank(Move::Left).unwrap();
        assert_eq!(puzzle.state, vec![vec![1, 0, 2], vec![4, 5, 3], vec![7, 8, 6]]);
        puzzle.move_blank(Move::Left).unwrap();
        assert_eq!(puzzle.state, vec![vec![0, 1, 2], vec![4, 5, 3], vec![7, 8, 6]]);
        assert!(puzzle.move_blank(Move::Left).is_err());
        puzzle.move_blank(Move::Down).unwrap();
        assert_eq!(puzzle.state, vec![vec![4, 1, 2], vec![0, 5, 3], vec![7, 8, 6]]);
        puzzle.move_blank(Move::Right).unwrap();
        assert_eq!(puzzle.state, vec![vec![4, 1, 2], vec![5, 0, 3], vec![7, 8, 6]]);
    }

    #[test]
    fn test_display() {
        let puzzle = Puzzle::new_answer(3);
        assert_eq!(format!("{}", puzzle), "1 2 3\n4 5 6\n7 8 0\n");
    }
}
