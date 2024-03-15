use crate::{Pos, Puzzle};

/// Inversion distance
///
/// # Reference
///
/// https://michael.kim/blog/puzzle
pub fn inversion_distance(puzzle: &Puzzle) -> usize {
    let size = puzzle.get_size();
    let flat_state = puzzle_to_vec_lr_tb(puzzle);
    let inversions = count_inversions(&flat_state);
    let vertical = calculate_move_lower_limit(inversions, size);
    let flat_state = puzzle_to_vec_tb_lr(puzzle);
    let inversions = count_inversions(&flat_state);
    let horizontal = calculate_move_lower_limit(inversions, size);
    vertical + horizontal
}

/// Convert puzzle to a vector from left to right and top to bottom
fn puzzle_to_vec_lr_tb(puzzle: &Puzzle) -> Vec<usize> {
    let size = puzzle.get_size();
    let answer_map = Puzzle::generate_arrange_order_answer_map(size, false);
    let mut flat_state = Vec::with_capacity(size * size - 1);
    for y in 0..size {
        for x in 0..size {
            if let Ok(value) = puzzle.get(Pos::new(x, y)) {
                if value != 0 {
                    let answer_pos = answer_map.get(&value).unwrap();
                    flat_state.push(*answer_pos);
                }
            }
        }
    }
    flat_state
}

/// Convert puzzle to a vector from top to bottom and left to right
fn puzzle_to_vec_tb_lr(puzzle: &Puzzle) -> Vec<usize> {
    let size = puzzle.get_size();
    let answer_map = Puzzle::generate_arrange_order_answer_map(size, false);
    let mut value_map = vec![0; size * size + 1];
    let mut count = 1;
    for x in 0..size {
        for y in 0..size {
            let index = y * size + x + 1;
            value_map[index] = count;
            count += 1;
        }
    }

    let mut flat_state = Vec::with_capacity(size * size - 1);
    for x in 0..size {
        for y in 0..size {
            if let Ok(value) = puzzle.get(Pos::new(x, y)) {
                if value != 0 {
                    let answer_pos = answer_map.get(&value).unwrap();
                    flat_state.push(value_map[*answer_pos]);
                }
            }
        }
    }
    flat_state
}

fn count_inversions(flat_state: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..flat_state.len() {
        for j in i + 1..flat_state.len() {
            if flat_state[i] > flat_state[j] {
                count += 1;
            }
        }
    }
    count
}

fn calculate_move_lower_limit(mut inversions: usize, size: usize) -> usize {
    let mut modulo = size - 1;
    let mut count = 0;
    while modulo > 0 {
        count += inversions / modulo;
        inversions %= modulo;
        if modulo < 2 {
            break;
        }
        modulo -= 2;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_puzzle_to_vec_lr_tb_trivial() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]])?;
        let flat_state = puzzle_to_vec_lr_tb(&puzzle);
        assert_eq!(flat_state, vec![1, 2, 3, 4, 6, 7, 8, 9]);
        Ok(())
    }

    #[test]
    fn test_puzzle_to_vec_lr_tb_normal() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 0, 3], vec![2, 8, 4], vec![7, 5, 6]])?;
        let flat_state = puzzle_to_vec_lr_tb(&puzzle);
        assert_eq!(flat_state, vec![1, 3, 2, 4, 6, 7, 9, 8]);
        Ok(())
    }

    #[test]
    fn test_puzzle_to_vec_tb_lr_trivial() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]])?;
        let flat_state = puzzle_to_vec_tb_lr(&puzzle);
        assert_eq!(flat_state, vec![1, 2, 3, 4, 6, 7, 8, 9]);
        Ok(())
    }

    #[test]
    fn test_puzzle_to_vec_tb_lr_normal() -> Result<()> {
        let puzzle = Puzzle::new_from_state(vec![vec![2, 1, 3], vec![8, 4, 0], vec![7, 5, 6]])?;
        let flat_state = puzzle_to_vec_tb_lr(&puzzle);
        assert_eq!(flat_state, vec![4, 2, 3, 1, 8, 9, 7, 6]);
        Ok(())
    }

    #[test]
    fn test_count_inversions_trivial() {
        let flat_state = vec![1, 2, 3, 4, 6, 7, 8, 9];
        let count = count_inversions(&flat_state);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_count_inversions_normal() {
        let flat_state = vec![1, 9, 2, 5, 3, 8, 7, 6];
        let count = count_inversions(&flat_state);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_calculate_move_lower_limit_3() {
        let inversions = 10;
        let size = 3;
        let count = calculate_move_lower_limit(inversions, size);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_calculate_move_lower_limit_4() {
        let inversions = 10;
        let size = 4;
        let count = calculate_move_lower_limit(inversions, size);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_calculate_move_lower_limit_5() {
        let inversions = 18;
        let size = 5;
        let count = calculate_move_lower_limit(inversions, size);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_calculate_move_lower_limit_6() {
        let inversions = 19;
        let size = 6;
        let count = calculate_move_lower_limit(inversions, size);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_inversion_distance_trivial() {
        let puzzle =
            Puzzle::new_from_state(vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]]).unwrap();
        let distance = inversion_distance(&puzzle);
        assert_eq!(distance, 0);
    }

    #[test]
    fn test_inversion_distance_normal() {
        let puzzle =
            Puzzle::new_from_state(vec![vec![2, 8, 3], vec![1, 0, 4], vec![7, 6, 5]]).unwrap();
        let distance = inversion_distance(&puzzle);
        assert_eq!(distance, 4);
    }
}
