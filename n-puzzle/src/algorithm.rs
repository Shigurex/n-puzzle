mod astar;
mod uniform_cost;
mod greedy;

use anyhow::Result;
use super::Puzzle;

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
pub struct Output {}

impl Output {
    pub fn new() -> Self {
        Self {}
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

    pub fn solve(&self) -> Result<()> {
        let output = match self.algorithm {
            Algorithm::AStar => astar::solve(&self.start_state, self.heuristic)?,
            Algorithm::UniformCost => uniform_cost::solve(&self.start_state)?,
            Algorithm::Greedy => greedy::solve(&self.start_state)?,
        };
        self.put_result(output);
        Ok(())
    }

    fn put_result(&self, output: Output) {
        println!("{:?}", output);
    }
}
