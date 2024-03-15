mod inversion_distance;

pub use inversion_distance::inversion_distance;

use crate::{Pos, Puzzle};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Heuristic {
    Manhattan,
    Hamming,
    LinearConflict,
    InversionDistance,
    None,
}

impl Heuristic {
    pub fn get_heuristic(&self) -> Result<fn(&Puzzle) -> usize> {
        let func = match self {
            Heuristic::Manhattan => manhattan,
            Heuristic::Hamming => hamming,
            Heuristic::LinearConflict => linear_conflict,
            Heuristic::InversionDistance => inversion_distance,
            Heuristic::None => return Err(anyhow!("Heuristic not set")),
        };
        Ok(func)
    }
}

// calculate manhattan distance
pub fn manhattan(puzzle: &Puzzle) -> usize {
    let size = puzzle.get_size();
    let answer_map = Puzzle::generate_answer_pos_map(size);
    let mut distance = 0;
    for i in 0..size * size {
        let puzzle_pos = Pos::new(i % size, i / size);
        if let Ok(puzzle_value) = puzzle.get(puzzle_pos) {
            let answer_pos = answer_map.get(&puzzle_value).unwrap();
            distance += (puzzle_pos.x as isize - answer_pos.x as isize).unsigned_abs()
                + (puzzle_pos.y as isize - answer_pos.y as isize).unsigned_abs();
        }
    }
    distance
}

// calculate hamming distance
pub fn hamming(puzzle: &Puzzle) -> usize {
    let size = puzzle.get_size();
    let answer = Puzzle::new_answer(size);
    let mut distance = 0;
    for i in 0..size * size {
        let puzzle_pos = Pos::new(i % size, i / size);
        if let Ok(puzzle_value) = puzzle.get(puzzle_pos) {
            let answer_value = answer.get(puzzle_pos).unwrap();
            if puzzle_value != answer_value {
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
    let answer_map = Puzzle::generate_answer_pos_map(size);

    for i in 0..size {
        let base_pos = Pos::new(i, row);
        let base_value = puzzle.get(base_pos).unwrap();
        if !puzzle.is_in_final_row(base_pos) {
            continue;
        }
        for j in i + 1..size {
            let comparison_pos = Pos::new(j, row);
            let comparison_value = puzzle.get(comparison_pos).unwrap();
            if !puzzle.is_in_final_row(comparison_pos) {
                continue;
            }
            if answer_map.get(&base_value).unwrap().x > answer_map.get(&comparison_value).unwrap().x
            {
                conflicts += 1;
            }
        }
    }
    conflicts
}

fn count_col_conflicts(puzzle: &Puzzle, col: usize) -> usize {
    let size = puzzle.get_size();
    let mut conflicts = 0;
    let answer_map = Puzzle::generate_answer_pos_map(size);

    for i in 0..size {
        let base_pos = Pos::new(col, i);
        let base_value = puzzle.get(base_pos).unwrap();
        if !puzzle.is_in_final_col(base_pos) {
            continue;
        }
        for j in i + 1..size {
            let comparison_pos = Pos::new(col, j);
            let comparison_value = puzzle.get(comparison_pos).unwrap();
            if !puzzle.is_in_final_col(comparison_pos) {
                continue;
            }
            if answer_map.get(&base_value).unwrap().y > answer_map.get(&comparison_value).unwrap().y
            {
                conflicts += 1;
            }
        }
    }
    conflicts
}
