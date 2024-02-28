use super::Output;
use crate::{Move, Puzzle};
use anyhow::Result;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Debug)]
struct Node {
    state: Puzzle,
    cost: usize,
    path: Vec<Move>,
}

impl Node {
    pub fn new(state: Puzzle, cost: usize, path: Vec<Move>) -> Self {
        Self { state, cost, path }
    }

    pub fn cost(&self) -> usize {
        self.cost
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

#[derive(Clone, Debug)]
struct UniformCostNode {
    node: Node,
}

impl UniformCostNode {
    pub fn new(node: Node) -> Self {
        Self { node }
    }
}

impl PartialEq for UniformCostNode {
    fn eq(&self, other: &Self) -> bool {
        self.node.cost() == other.node.cost()
    }
}

impl Eq for UniformCostNode {}

impl PartialOrd for UniformCostNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UniformCostNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.node.cost().cmp(&self.node.cost())
    }
}

#[derive(Debug)]
struct OpendSet {
    set: BinaryHeap<UniformCostNode>,
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

    pub fn insert(&mut self, node: Node) {
        self.set.push(UniformCostNode::new(node));
        self.count += 1;
        if self.set.len() > self.max_size {
            self.max_size = self.set.len();
        }
    }

    pub fn pop(&mut self) -> Option<Node> {
        if let Some(UniformCostNode { node }) = self.set.pop() {
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

fn uniform_cost(puzzle: Puzzle) -> Result<Output> {
    let mut open_set = OpendSet::new();
    let mut closed_set = ClosedSet::new();
    open_set.insert(Node::new(puzzle, 0, vec![]));
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
                    open_set.insert(Node::new(new_state, node.cost() + 1, new_path));
                }
            }
        }
    }
    Err(anyhow::anyhow!("No solution"))
}

pub(super) fn solve(puzzle: &Puzzle) -> Result<Output> {
    uniform_cost(puzzle.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_cost() {
        let mut puzzle = Puzzle::generate_solvable(3).unwrap();
        println!("{:?}", puzzle);
        let output = uniform_cost(puzzle.clone()).unwrap();
        for m in output.path {
            puzzle.move_blank(m).unwrap();
        }
        assert!(puzzle.is_final_state());
    }
}
