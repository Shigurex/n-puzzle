use crate::{Pos, Puzzle};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Heuristic {
    Manhattan,
    Hamming,
    LinearConflict,
    None,
}

impl Heuristic {
    pub fn get_heuristic(&self) -> Result<fn(&Puzzle) -> usize> {
        let func = match self {
            Heuristic::Manhattan => manhattan,
            Heuristic::Hamming => hamming,
            Heuristic::LinearConflict => linear_conflict,
            Heuristic::None => return Err(anyhow!("Heuristic not set")),
        };
        Ok(func)
    }
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
pub fn hamming(puzzle: &Puzzle) -> usize {
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
pub fn linear_conflict(puzzle: &Puzzle) -> usize {
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
