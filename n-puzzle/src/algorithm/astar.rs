use super::{ClosedSet, Heuristic, OpenSet, OpenSetNode, Output};
use crate::{n_puzzle::Pos, Move, Puzzle};
use anyhow::{anyhow, Result};

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

// calculate manhattan distance
pub fn manhattan(puzzle: &Puzzle) -> usize {
    let size = puzzle.get_size();
    let mut distance = 0;

    for i in 0..size * size {
        let puzzle_pos = Pos::new(i % size, i / size);
        if let Ok(puzzle_value) = puzzle.get(puzzle_pos) {
            let answer_pos = Pos::new(
                ((puzzle_value + size * size - 1) % (size * size)) % size,
                ((puzzle_value + size * size - 1) % (size * size)) / size,
            );
            distance += puzzle_pos.x.abs_diff(answer_pos.x) + puzzle_pos.y.abs_diff(answer_pos.y);
        }
    }
    distance
}

// calculate hamming distance
fn hamming(puzzle: &Puzzle) -> usize {
    let size = puzzle.get_size();
    let mut distance = 0;

    for i in 0..size * size {
        let puzzle_pos = Pos::new(i % size, i / size);
        if let Ok(puzzle_value) = puzzle.get(puzzle_pos) {
            let answer_pos = Pos::new(
                ((puzzle_value + size * size - 1) % (size * size)) % size,
                ((puzzle_value + size * size - 1) % (size * size)) / size,
            );
            if puzzle_pos != answer_pos {
                distance += 1;
            }
        }
    }
    distance
}

// https://medium.com/swlh/looking-into-k-puzzle-heuristics-6189318eaca2
// Two tiles t_j and t_k are in linear conflict if t_j and t_k are in the same line,
// the goal position of t_j and t_k are both in that line, t_j is to the right of t_k,
// and the goal position of t_j is to the left of the goal position of t_k.
fn linear_conflict(puzzle: &Puzzle) -> usize {
    let size = puzzle.get_size();
    let mut distance = manhattan(puzzle);
    let mut conflicts = 0;

    for i in 0..size {
        conflicts += count_row_conflicts(puzzle, i);
        conflicts += count_col_conflicts(puzzle, i);
    }
    distance += conflicts * 2;

    distance
}

fn count_row_conflicts(puzzle: &Puzzle, row: usize) -> usize {
    let size = puzzle.get_size();
    let mut conflicts = 0;

    for i in 0..size {
        let base_pos = Pos::new(i, row);
        if !puzzle.is_in_final_row(base_pos) {
            continue;
        }
        for j in i + 1..size {
            let comparison_pos = Pos::new(j, row);
            if !puzzle.is_in_final_row(comparison_pos) {
                continue;
            }
            if puzzle.get(base_pos).unwrap() > puzzle.get(comparison_pos).unwrap() {
                conflicts += 1;
            }
        }
    }
    conflicts
}

fn count_col_conflicts(puzzle: &Puzzle, col: usize) -> usize {
    let size = puzzle.get_size();
    let mut conflicts = 0;

    for i in 0..size {
        let base_pos = Pos::new(col, i);
        if !puzzle.is_in_final_col(base_pos) {
            continue;
        }
        for j in i + 1..size {
            let comparison_pos = Pos::new(col, j);
            if !puzzle.is_in_final_col(comparison_pos) {
                continue;
            }
            if puzzle.get(base_pos).unwrap() > puzzle.get(comparison_pos).unwrap() {
                conflicts += 1;
            }
        }
    }
    conflicts
}

pub(super) fn solve(puzzle: &Puzzle, heuristic: Heuristic) -> Result<Output> {
    match heuristic {
        Heuristic::Manhattan => astar(puzzle.clone(), manhattan),
        Heuristic::Hamming => astar(puzzle.clone(), hamming),
        Heuristic::LinearConflict => astar(puzzle.clone(), linear_conflict),
        _ => Err(anyhow!("Heuristics not set for astar.")),
    }
}
