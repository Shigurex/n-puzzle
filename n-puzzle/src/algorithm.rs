mod astar;
mod uniform_cost;
mod greedy;

use anyhow::Result;
use super::{Puzzle, Move};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Heuristic {
    Manhattan,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Algorithm {
    AStar,
    UniformCost,
    Greedy,
}

impl Algorithm {
    pub fn is_heuristic(&self) -> bool {
        if let Algorithm::UniformCost = self {
            return false
        }
        return true
    }
}

#[derive(Debug)]
pub struct Output {
    pub complexity_in_time: usize,
    pub complexity_in_size: usize,
    pub path: Vec<Move>,
}

impl Output {
    pub fn new(complexity_in_time: usize, complexity_in_size: usize, path: Vec<Move>) -> Self {
        Self {
            complexity_in_time,
            complexity_in_size,
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

    fn non_verbose_output(&self, mut text: String) -> String {
        for m in &self.path {
            text += format!("{:?} ", m).as_str();
        }
        text += "\n";
        text
    }

    pub fn get_result_string(&self, puzzle: Option<Puzzle>) -> Result<String> {
        let mut text = String::new();
        text += format!("Complexity in time: {}\n", self.complexity_in_time).as_str();
        text += format!("Complexity in size: {}\n", self.complexity_in_size).as_str();
        text += format!("Number of moves: {}\n", self.path.len()).as_str();
        let text = if let Some(puzzle) = puzzle {
            self.verbose_output(text, puzzle)?
        } else {
            self.non_verbose_output(text)
        };
        Ok(text)
    }

    pub fn put_result(&self, puzzle: Option<Puzzle>) -> Result<()> {
        let text = self.get_result_string(puzzle)?;
        print!("{}", text);
        Ok(())
    }
}

pub struct Solver {
    algorithm: Algorithm,
    heuristic: Heuristic,
    start_state: Puzzle,
}

impl Solver {
    pub fn new(algorithm: Algorithm, heuristic: Heuristic, start_state: Puzzle) -> Self {
        Self {
            algorithm,
            heuristic,
            start_state,
        }
    }

    pub fn solve(&self, verbose: bool) -> Result<()> {
        let output = match self.algorithm {
            Algorithm::AStar => astar::solve(&self.start_state, self.heuristic)?,
            Algorithm::UniformCost => uniform_cost::solve(&self.start_state)?,
            Algorithm::Greedy => greedy::solve(&self.start_state)?,
        };
        self.put_result(output, verbose)?;
        Ok(())
    }

    fn put_result(&self, output: Output, verbose: bool) -> Result<()> {
        if verbose {
            output.put_result(Some(self.start_state.clone()))
        } else {
            output.put_result(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_verbose() -> Result<()> {
        let output = Output::new(
            1,
            1,
            vec![Move::Right]
        );
        let puzzle = Puzzle::new_from_state(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 0, 8],
            ]
        )?;
        let text = output.get_result_string(Some(puzzle))?;
        assert_eq!(text,
"Complexity in time: 1
Complexity in size: 1
Number of moves: 1
1 2 3
4 5 6
7 0 8
↓ Right
1 2 3
4 5 6
7 8 0
");
        Ok(())
    }

    #[test]
    fn test_output_non_verbose() -> Result<()> {
        let output = Output::new(
            1,
            1,
            vec![Move::Up, Move::Left]
        );
        let text = output.get_result_string(None)?;
        assert_eq!(text,
"Complexity in time: 1
Complexity in size: 1
Number of moves: 2
Up Left 
");
        Ok(())
    }
}
