use crate::{Move, Puzzle};
use anyhow::Result;

#[derive(Debug)]
pub struct Output {
    pub complexity_in_time: usize,
    pub complexity_in_size: usize,
    pub elapsed_time: f64,
    pub path: Vec<Move>,
}

impl Output {
    pub fn new(
        complexity_in_time: usize,
        complexity_in_size: usize,
        elapsed_time: f64,
        path: Vec<Move>,
    ) -> Self {
        Self {
            complexity_in_time,
            complexity_in_size,
            elapsed_time,
            path,
        }
    }

    fn verbose_output(&self, mut text: String, mut puzzle: Puzzle) -> Result<String> {
        text += format!("{}", puzzle).as_str();
        for m in &self.path {
            puzzle.move_blank(*m)?;
            text += format!("↓ {:?}\n", m).as_str();
            text += format!("{}", puzzle).as_str();
        }
        Ok(text)
    }

    fn non_verbose_output(&self, mut text: String, puzzle: Puzzle) -> String {
        text += format!("{}", puzzle).as_str();
        text += "Moves: ";
        for m in &self.path {
            text += format!("{:?} ", m).as_str();
        }
        text += "\n";
        text
    }

    pub fn get_result_string(&self, puzzle: Puzzle, verbose: bool) -> Result<String> {
        let mut text = String::new();
        text += format!("Complexity in time: {}\n", self.complexity_in_time).as_str();
        text += format!("Complexity in size: {}\n", self.complexity_in_size).as_str();
        text += format!("Elapsed time: {:.6} seconds\n", self.elapsed_time).as_str();
        text += format!("Number of moves: {}\n", self.path.len()).as_str();
        let text = if verbose {
            self.verbose_output(text, puzzle)?
        } else {
            self.non_verbose_output(text, puzzle)
        };
        Ok(text)
    }

    pub fn put_result(&self, puzzle: Puzzle, verbose: bool) -> Result<()> {
        let text = self.get_result_string(puzzle, verbose)?;
        print!("{}", text);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_verbose() -> Result<()> {
        let output = Output::new(1, 1, 1.0, vec![Move::Right]);
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 0, 8]])?;
        let text = output.get_result_string(puzzle, true)?;
        assert_eq!(
            text,
            "Complexity in time: 1
Complexity in size: 1
Elapsed time: 1.000000 seconds
Number of moves: 1
1 2 3
4 5 6
7 0 8
↓ Right
1 2 3
4 5 6
7 8 0
"
        );
        Ok(())
    }

    #[test]
    fn test_output_non_verbose() -> Result<()> {
        let output = Output::new(1, 1, 2.0, vec![Move::Up, Move::Left]);
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![4, 0, 5], vec![7, 8, 6]])?;
        let text = output.get_result_string(puzzle, false)?;
        assert_eq!(
            text,
            "Complexity in time: 1
Complexity in size: 1
Elapsed time: 2.000000 seconds
Number of moves: 2
1 2 3
4 0 5
7 8 6
Moves: Up Left 
"
        );
        Ok(())
    }
}
