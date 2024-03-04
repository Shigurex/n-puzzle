mod astar;
mod closed_set;
mod greedy;
mod open_set;
mod output;
mod uniform_cost;

use astar::astar;
use closed_set::ClosedSet;
use open_set::{OpenSet, OpenSetNode};
use output::Output;

use super::Puzzle;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Heuristic {
    Manhattan,
    Hamming,
    LinearConflict,
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
            return false;
        }
        true
    }
}

pub struct Solver {
    algorithm: Algorithm,
    heuristic: Heuristic,
    start_state: Puzzle,
    timeout: Option<u64>,
}

impl Solver {
    pub fn new(
        algorithm: Algorithm,
        heuristic: Heuristic,
        start_state: Puzzle,
        timeout: Option<u64>,
    ) -> Self {
        Self {
            algorithm,
            heuristic,
            start_state,
            timeout,
        }
    }

    pub fn solve(&self, verbose: bool) -> Result<()> {
        let output = match self.algorithm {
            Algorithm::AStar => astar::solve(&self.start_state, self.heuristic, self.timeout)?,
            Algorithm::UniformCost => uniform_cost::solve(&self.start_state, self.timeout)?,
            Algorithm::Greedy => greedy::solve(&self.start_state, self.heuristic, self.timeout)?,
        };
        self.put_result(output, verbose)?;
        Ok(())
    }

    fn put_result(&self, output: Output, verbose: bool) -> Result<()> {
        output.put_result(self.start_state.clone(), verbose)
    }
}
