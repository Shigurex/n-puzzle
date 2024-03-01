use super::Output;
use crate::{Move, Puzzle};
use anyhow::Result;
use std::collections::{BinaryHeap, HashSet};

fn uniform_cost(_puzzle: &Puzzle) -> usize {
    0
}

#[derive(Clone, Debug)]
struct OpendSetNode {
    state: Puzzle,
    path: Vec<Move>,
    cost: usize,
    heuristics_cost: usize,
}

impl OpendSetNode {
    pub fn new(state: Puzzle, path: Vec<Move>, cost: usize, heuristic: fn (&Puzzle) -> usize) -> Self {
        let heuristics_cost = heuristic(&state);
        Self {
            state,
            path,
            cost,
            heuristics_cost,
        }
    }

    pub fn cost(&self) -> usize {
        self.cost
    }

    pub fn heuristics_cost(&self) -> usize {
        self.heuristics_cost
    }

    pub fn total_cost(&self) -> usize {
        self.cost + self.heuristics_cost
    }

    pub fn is_goal(&self) -> bool {
        self.state.is_final_state()
    }

    pub fn state(&self) -> &Puzzle {
        &self.state
    }

    pub fn path(&self) -> &Vec<Move> {
        &self.path
    }
}

impl PartialEq for OpendSetNode {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost() == other.total_cost()
    }
}

impl Eq for OpendSetNode {}

impl PartialOrd for OpendSetNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpendSetNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.total_cost()).cmp(&self.total_cost())
    }
}

#[derive(Debug)]
struct OpendSet {
    set: BinaryHeap<OpendSetNode>,
    max_size: usize,
    count: usize,
}

impl OpendSet {
    pub fn new() -> Self {
        Self {
            set: BinaryHeap::new(),
            max_size: 0,
            count: 0,
        }
    }

    pub fn get_append_count(&self) -> usize {
        self.count
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }

    pub fn insert(&mut self, node: OpendSetNode) {
        self.set.push(node);
        self.count += 1;
        if self.set.len() > self.max_size {
            self.max_size = self.set.len();
        }
    }

    pub fn pop(&mut self) -> Option<OpendSetNode> {
        if let Some(node) = self.set.pop() {
            Some(node)
        } else {
            None
        }
    }
}

struct ClosedSet {
    set: HashSet<Puzzle>,
}

impl ClosedSet {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, state: Puzzle) {
        self.set.insert(state);
    }

    pub fn contains(&self, state: &Puzzle) -> bool {
        self.set.contains(state)
    }
}

fn astar(puzzle: Puzzle, heuristic: fn (&Puzzle) -> usize) -> Result<Output> {
    let mut open_set = OpendSet::new();
    let mut closed_set = ClosedSet::new();
    open_set.insert(OpendSetNode::new(puzzle, vec![], 0, heuristic));
    while let Some(node) = open_set.pop() {
        if node.is_goal() {
            return Ok(Output::new(
                open_set.get_append_count(),
                open_set.get_max_size(),
                node.path,
            ));
        }
        closed_set.insert(node.state().clone());
        for move_dir in Move::list() {
            let mut new_state = node.state().clone();
            if let Ok(()) = new_state.move_blank(move_dir) {
                if !closed_set.contains(&new_state) {
                    let mut new_path = node.path().clone();
                    new_path.push(move_dir);
                    open_set.insert(OpendSetNode::new(
                        new_state,
                        new_path,
                        node.cost() + 1,
                        heuristic,
                    ));
                }
            }
        }
    }
    Err(anyhow::anyhow!("No solution"))
}

pub(super) fn solve(puzzle: &Puzzle) -> Result<Output> {
    astar(puzzle.clone(), uniform_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_cost() {
        let mut puzzle = Puzzle::generate_solvable(3).unwrap();
        println!("{:?}", puzzle);
        let output = astar(puzzle.clone(), uniform_cost).unwrap();
        for m in output.path {
            puzzle.move_blank(m).unwrap();
        }
        assert!(puzzle.is_final_state());
    }
}
