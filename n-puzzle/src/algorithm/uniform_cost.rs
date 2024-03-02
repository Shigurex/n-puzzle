use super::{ClosedSet, OpenSet, OpenSetNode, Output};
use crate::{Move, Puzzle};
use anyhow::Result;

fn uniform_cost(_puzzle: &Puzzle) -> usize {
    0
}

fn astar(puzzle: Puzzle, heuristic: fn(&Puzzle) -> usize) -> Result<Output> {
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
