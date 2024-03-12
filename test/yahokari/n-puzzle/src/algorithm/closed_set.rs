use crate::Puzzle;
use std::collections::HashSet;

pub struct ClosedSet {
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
