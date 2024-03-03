use crate::{Move, Puzzle};
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
pub struct OpenSetNode {
    state: Puzzle,
    path: Vec<Move>,
    moved_cost: usize,
    heuristics_cost: usize,
}

impl OpenSetNode {
    pub fn new(
        state: Puzzle,
        path: Vec<Move>,
        moved_cost: usize,
        heuristic: fn(&Puzzle) -> usize,
    ) -> Self {
        let heuristics_cost = heuristic(&state);
        Self {
            state,
            path,
            moved_cost,
            heuristics_cost,
        }
    }

    pub fn moved_cost(&self) -> usize {
        self.moved_cost
    }

    pub fn heuristics_cost(&self) -> usize {
        self.heuristics_cost
    }

    pub fn total_cost(&self) -> usize {
        self.moved_cost + self.heuristics_cost
    }

    pub fn is_goal(&self) -> bool {
        self.state.is_final_state()
    }

    pub fn state(&self) -> &Puzzle {
        &self.state
    }

    pub fn to_state(self) -> Puzzle {
        self.state
    }

    pub fn path(&self) -> &Vec<Move> {
        &self.path
    }
}

impl PartialEq for OpenSetNode {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost() == other.total_cost()
    }
}

impl Eq for OpenSetNode {}

impl PartialOrd for OpenSetNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpenSetNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.total_cost()).cmp(&self.total_cost())
    }
}

#[derive(Debug)]
pub struct OpenSet {
    set: BinaryHeap<OpenSetNode>,
    max_size: usize,
    count: usize,
}

impl OpenSet {
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

    pub fn insert(&mut self, node: OpenSetNode) {
        self.set.push(node);
        self.count += 1;
        if self.set.len() > self.max_size {
            self.max_size = self.set.len();
        }
    }

    pub fn pop(&mut self) -> Option<OpenSetNode> {
        self.set.pop()
    }
}
