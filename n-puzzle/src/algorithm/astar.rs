use super::{ClosedSet, Heuristic, OpenSet, OpenSetNode, Output};
use crate::{Move, Puzzle};
use anyhow::Result;
use std::time::{Duration, Instant};

fn append_optimal_state(
    open_set: &mut OpenSet,
    node: &OpenSetNode,
    heuristic: fn(&Puzzle) -> usize,
) -> Result<()> {
    let mut score = node.heuristics_cost();
    let mut optimal_puzzle = None;
    let mut optimal_direction = None;
    for move_dir in Move::list() {
        let mut new_state = node.state().clone();
        if let Ok(()) = new_state.move_blank(move_dir) {
            let new_score = heuristic(&new_state);
            if new_score < score {
                score = new_score;
                optimal_puzzle = Some(new_state);
                optimal_direction = Some(move_dir);
            }
        }
    }
    if let Some(puzzle) = optimal_puzzle {
        let mut new_path = node.path().clone();
        new_path.push(optimal_direction.unwrap());
        open_set.insert(OpenSetNode::new(
            puzzle,
            new_path,
            node.moved_cost() + 1,
            heuristic,
        ));
        Ok(())
    } else {
        Err(anyhow::anyhow!("Optimal state not found"))
    }
}

fn append_all_movable_states(
    open_set: &mut OpenSet,
    closed_set: &ClosedSet,
    node: &OpenSetNode,
    heuristic: fn(&Puzzle) -> usize,
) {
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

pub fn astar(
    puzzle: Puzzle,
    heuristic: fn(&Puzzle) -> usize,
    is_greedy: bool,
    timeout: Option<u64>,
) -> Result<Output> {
    let mut open_set = OpenSet::new();
    let mut closed_set = ClosedSet::new();
    open_set.insert(OpenSetNode::new(puzzle, vec![], 0, heuristic));
    let mut max_size = 0;

    let start = Instant::now();
    let timeout = timeout.map(|t| Duration::new(t, 0));
    while let Some(node) = open_set.pop() {
        if let Some(duration) = timeout {
            if start.elapsed() > duration {
                return Err(anyhow::anyhow!("Timeout"));
            }
        }
        if node.is_goal() {
            return Ok(Output::new(
                open_set.get_append_count(),
                max_size,
                start.elapsed().as_secs_f64(),
                node.path().clone(),
            ));
        }
        if is_greedy {
            append_optimal_state(&mut open_set, &node, heuristic)?;
        } else {
            append_all_movable_states(&mut open_set, &closed_set, &node, heuristic);
            closed_set.insert(node.convert_to_state());
        }
        // max_size <= open_set.len() + closed_set.len()
        max_size = open_set.len() + closed_set.len();
    }
    Err(anyhow::anyhow!("No solution"))
}

pub(super) fn solve(puzzle: &Puzzle, heuristic: Heuristic, timeout: Option<u64>) -> Result<Output> {
    astar(puzzle.clone(), heuristic.get_heuristic()?, false, timeout)
}
