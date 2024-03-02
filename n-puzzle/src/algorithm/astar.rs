use super::{ClosedSet, Heuristic, OpenSet, OpenSetNode, Output};
use crate::{Move, Puzzle};
use anyhow::Result;

pub fn astar(puzzle: Puzzle, heuristic: fn(&Puzzle) -> usize) -> Result<Output> {
    let mut open_set = OpenSet::new();
    let mut closed_set = ClosedSet::new();
    open_set.insert(OpenSetNode::new(puzzle, vec![], 0, heuristic));
    while let Some(node) = open_set.pop() {
        if node.is_goal() {
            return Ok(Output::new(
                open_set.get_append_count(),
                open_set.get_max_size(),
                node.path().clone(),
            ));
        }
        closed_set.insert(node.state().clone());
        for move_dir in Move::list() {
            let mut new_state = node.state().clone();
            if let Ok(()) = new_state.move_blank(move_dir) {
                if !closed_set.contains(&new_state) {
                    let mut new_path = node.path().clone();
                    new_path.push(move_dir);
                    open_set.insert(OpenSetNode::new(
                        new_state,
                        new_path,
                        node.moved_cost() + 1,
                        heuristic,
                    ));
                }
            }
        }
    }
    Err(anyhow::anyhow!("No solution"))
}

pub(super) fn solve(_puzzle: &Puzzle, _heuristic: Heuristic) -> Result<Output> {
    Ok(Output::new(0, 0, vec![]))
}
