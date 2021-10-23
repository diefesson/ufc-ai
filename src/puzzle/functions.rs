use std::iter::once;

use crate::puzzle::*;

pub fn puzzle_expander<const S: usize>(state: &PuzzleState<S>) -> Vec<PuzzleState<S>> {
    let options = vec![
        state.move_tile(UP),
        state.move_tile(DOWN),
        state.move_tile(LEFT),
        state.move_tile(RIGHT),
    ];
    options
        .into_iter()
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect()
}

pub fn puzzle_verifier<const S: usize>(state: &PuzzleState<S>) -> bool {
    let correct = 1..=S * S;
    let found = state.numbers().iter().flatten();
    correct.zip(found).all(|(n0, n1)| n0 as i32 == *n1)
}
