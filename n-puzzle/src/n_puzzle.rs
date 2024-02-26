mod generator;
mod parser;
mod pos;
mod solvable;

pub use pos::Pos;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub enum PuzzleSettings {
    Size(usize),
    TextPath(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Puzzle {
    size: usize,
    state: Vec<Vec<usize>>,
    blank_pos: Pos,
}

impl Puzzle {
    /// Generate a new puzzle with PuzzleSettings
    pub fn new(settings: PuzzleSettings) -> Result<Self> {
        match settings {
            PuzzleSettings::Size(size) => Self::generate_solvable(size),
            PuzzleSettings::TextPath(text_path) => Self::parse_text(text_path),
        }
    }

    pub fn new_from_state(state: Vec<Vec<usize>>) -> Result<Self> {
        let size = state.len();
        let mut blank_pos = Pos::new(0, 0);
        'outer: for (y, row) in state.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == 0 {
                    blank_pos = Pos::new(x, y);
                    break 'outer;
                }
            }
        }
        let puzzle = Self {
            size,
            state,
            blank_pos,
        };
        if !puzzle.check_state() {
            return Err(anyhow!("Invalid state"));
        }
        Ok(puzzle)
    }

    /// Generate a answer puzzle with the given size
    pub fn new_answer(size: usize) -> Self {
        let mut state = vec![vec![0; size]; size];
        let mut count = 1;
        for row in &mut state {
            for val in row {
                *val = count;
                count += 1;
            }
        }
        state[size - 1][size - 1] = 0;
        Self {
            size,
            state,
            blank_pos: Pos::new(size - 1, size - 1),
        }
    }

    /// Check puzzle state
    pub fn check_state(&self) -> bool {
        let mut state = vec![false; self.size * self.size];
        if self.state.len() != self.size {
            return false;
        }
        for row in &self.state {
            if row.len() != self.size {
                return false;
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
        let val = match self.get(self.blank_pos) {
            Ok(val) => val,
            Err(_) => return false,
        };
        if val != 0 {
            return false;
        }
        true
    }

    /// Checl if the puzzle is in the final state
    pub fn is_final_state(&self) -> bool {
        let mut count = 1;
        for i in 0..self.size {
            for j in 0..self.size {
                if count == self.size * self.size {
                    break;
                }
                if self.state[i][j] != count {
                    return false;
                }
                count += 1;
            }
        }
        true
    }

    /// Get the value at the given position
    pub fn get(&self, pos: Pos) -> Result<usize> {
        if pos.x >= self.size || pos.y >= self.size {
            return Err(anyhow!("Index out of bounds: ({}, {})", pos.x, pos.y));
        }
        Ok(self.state[pos.y][pos.x])
    }

    /// Set the value at the given position without value checking
    pub fn unchecked_set(&mut self, pos: Pos, val: usize) -> Result<()> {
        if pos.x >= self.size || pos.y >= self.size {
            return Err(anyhow!("Index out of bounds: ({}, {})", pos.x, pos.y));
        }
        self.state[pos.y][pos.x] = val;
        if val == 0 {
            self.blank_pos = pos;
        }
        Ok(())
    }

    /// Set the value at the given position with value checking
    pub fn set(&mut self, pos: Pos, val: usize) -> Result<()> {
        if val >= self.size * self.size {
            return Err(anyhow!("Value out of bounds: {}", val));
        }
        for row in &self.state {
            if row.contains(&val) {
                return Err(anyhow!("Value already exists: {}", val));
            }
        }
        self.unchecked_set(pos, val)
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_blank_pos(&self) -> Pos {
        self.blank_pos
    }

    /// Swap the values at the given positions
    pub fn swap(&mut self, pos1: Pos, pos2: Pos) -> Result<()> {
        if pos1.x >= self.size || pos1.y >= self.size || pos2.x >= self.size || pos2.y >= self.size
        {
            return Err(anyhow!(
                "Index out of bounds: ({}, {}), ({}, {})",
                pos1.x,
                pos1.y,
                pos2.x,
                pos2.y
            ));
        }
        let val1 = self.get(pos1)?;
        let val2 = self.get(pos2)?;
        self.state[pos1.y][pos1.x] = val2;
        self.state[pos2.y][pos2.x] = val1;
        if val1 == 0 {
            self.blank_pos = pos2;
        } else if val2 == 0 {
            self.blank_pos = pos1;
        }
        Ok(())
    }

    /// Move the blank position
    pub fn move_blank(&mut self, mv: Move) -> Result<()> {
        let pos = self.blank_pos;
        match mv {
            Move::Up => {
                if pos.y == 0 {
                    return Err(anyhow!("Cannot move up"));
                }
                self.swap(pos, pos - Pos::new(0, 1))?;
            }
            Move::Down => {
                if pos.y == self.size - 1 {
                    return Err(anyhow!("Cannot move down"));
                }
                self.swap(pos, pos + Pos::new(0, 1))?;
            }
            Move::Left => {
                if pos.x == 0 {
                    return Err(anyhow!("Cannot move left"));
                }
                self.swap(pos, pos - Pos::new(1, 0))?;
            }
            Move::Right => {
                if pos.x == self.size - 1 {
                    return Err(anyhow!("Cannot move right"));
                }
                self.swap(pos, pos + Pos::new(1, 0))?;
            }
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
            writeln!(f)?;
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
        assert_eq!(
            puzzle.state,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]]
        );
    }

    #[test]
    fn test_check_state() {
        let mut puzzle = Puzzle::new_answer(3);
        assert!(puzzle.check_state());
        // Case where a value is duplicated
        puzzle.state[0][0] = 0;
        assert!(!puzzle.check_state());
        puzzle.state[0][0] = 1;
        // Case where the line length is not equal to the size
        puzzle.state.pop();
        puzzle.state.push(vec![8, 9]);
        assert!(!puzzle.check_state());
        puzzle.state.pop();
        puzzle.state.push(vec![8, 9, 0]);
        // Case where the column length is not equal to the size
        puzzle.state.push(vec![0; 3]);
        assert!(!puzzle.check_state());
        puzzle.state.pop();
        // Case where a value is greater than the size
        puzzle.state[0][0] = 9;
        assert!(!puzzle.check_state());
        puzzle.state[0][0] = 1;
        // Case where the blank position is not correct
        puzzle.blank_pos = Pos::new(0, 0);
        assert!(!puzzle.check_state());
    }

    #[test]
    fn test_is_final_state() {
        let mut puzzle = Puzzle::new_answer(3);
        assert!(puzzle.is_final_state());
        puzzle.state[0][0] = 0;
        assert!(!puzzle.is_final_state());
    }

    #[test]
    fn test_get() {
        let puzzle = Puzzle::new_answer(3);
        assert_eq!(puzzle.get(Pos::new(0, 0)).unwrap(), 1);
        assert_eq!(puzzle.get(Pos::new(1, 2)).unwrap(), 8);
        assert_eq!(puzzle.get(Pos::new(2, 2)).unwrap(), 0);
        assert!(puzzle.get(Pos::new(3, 0)).is_err());
        assert!(puzzle.get(Pos::new(0, 3)).is_err());
    }

    #[test]
    fn test_unchecked_set() {
        let mut puzzle = Puzzle::new_answer(3);
        puzzle.unchecked_set(Pos::new(0, 0), 9).unwrap();
        assert_eq!(puzzle.state[0][0], 9);
        puzzle.unchecked_set(Pos::new(2, 2), 1).unwrap();
        assert_eq!(puzzle.state[2][2], 1);
        puzzle.unchecked_set(Pos::new(0, 2), 0).unwrap();
        assert_eq!(puzzle.state[2][0], 0);
        assert_eq!(puzzle.blank_pos, Pos::new(0, 2));
        assert!(puzzle.unchecked_set(Pos::new(3, 0), 2).is_err());
        assert!(puzzle.unchecked_set(Pos::new(0, 3), 3).is_err());
    }

    #[test]
    fn test_set() {
        let mut puzzle = Puzzle {
            size: 3,
            state: vec![vec![0; 3]; 3],
            blank_pos: Pos::new(2, 2),
        };
        puzzle.set(Pos::new(0, 0), 1).unwrap();
        assert_eq!(puzzle.state[0][0], 1);
        puzzle.set(Pos::new(2, 2), 2).unwrap();
        assert_eq!(puzzle.state[2][2], 2);
        assert!(puzzle.set(Pos::new(0, 0), 2).is_err());
        assert!(puzzle.set(Pos::new(0, 0), 10).is_err());
        assert!(puzzle.set(Pos::new(0, 3), 3).is_err());
        assert!(puzzle.set(Pos::new(3, 0), 3).is_err());
    }

    #[test]
    fn test_swap() {
        let mut puzzle = Puzzle::new_answer(3);
        puzzle.swap(Pos::new(0, 0), Pos::new(2, 2)).unwrap();
        assert_eq!(puzzle.state[0][0], 0);
        assert_eq!(puzzle.state[2][2], 1);
        assert_eq!(puzzle.blank_pos, Pos::new(0, 0));
        puzzle.swap(Pos::new(0, 0), Pos::new(2, 2)).unwrap();
        assert_eq!(puzzle.state[0][0], 1);
        assert_eq!(puzzle.state[2][2], 0);
        assert_eq!(puzzle.blank_pos, Pos::new(2, 2));
        assert!(puzzle.swap(Pos::new(0, 3), Pos::new(0, 0)).is_err());
        assert!(puzzle.swap(Pos::new(0, 0), Pos::new(3, 0)).is_err());
    }

    #[test]
    fn test_move_blank() {
        let mut puzzle = Puzzle::new_answer(3);
        assert!(puzzle.move_blank(Move::Down).is_err());
        assert!(puzzle.move_blank(Move::Right).is_err());
        puzzle.move_blank(Move::Up).unwrap();
        assert_eq!(
            puzzle.state,
            vec![vec![1, 2, 3], vec![4, 5, 0], vec![7, 8, 6]]
        );
        puzzle.move_blank(Move::Up).unwrap();
        assert_eq!(
            puzzle.state,
            vec![vec![1, 2, 0], vec![4, 5, 3], vec![7, 8, 6]]
        );
        assert!(puzzle.move_blank(Move::Up).is_err());
        puzzle.move_blank(Move::Left).unwrap();
        assert_eq!(
            puzzle.state,
            vec![vec![1, 0, 2], vec![4, 5, 3], vec![7, 8, 6]]
        );
        puzzle.move_blank(Move::Left).unwrap();
        assert_eq!(
            puzzle.state,
            vec![vec![0, 1, 2], vec![4, 5, 3], vec![7, 8, 6]]
        );
        assert!(puzzle.move_blank(Move::Left).is_err());
        puzzle.move_blank(Move::Down).unwrap();
        assert_eq!(
            puzzle.state,
            vec![vec![4, 1, 2], vec![0, 5, 3], vec![7, 8, 6]]
        );
        puzzle.move_blank(Move::Right).unwrap();
        assert_eq!(
            puzzle.state,
            vec![vec![4, 1, 2], vec![5, 0, 3], vec![7, 8, 6]]
        );
    }

    #[test]
    fn test_display() {
        let puzzle = Puzzle::new_answer(3);
        assert_eq!(format!("{}", puzzle), "1 2 3\n4 5 6\n7 8 0\n");
    }
}
