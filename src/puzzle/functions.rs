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

pub fn wrong_place_heuristic<const S: usize>(state: &PuzzleState<S>) -> f64{
    let mut count = 0.0;
    for i in 0..S{
        for j in 0..S{
            let n = state.numbers()[i][j];
            if (i, j) != PuzzleState::<S>::correct_pos(n){
                count += 1.0;
            }
        }
    }
    return count;
}

pub fn manhattan_heuristic<const S: usize>(state: &PuzzleState<S>) -> f64{
    let mut distance = 0.0;
    for i in 0..S{
        for j in 0..S{
            let n = state.numbers()[i][j];
            let (correct_i, correct_j) = PuzzleState::<S>::correct_pos(n);
            distance += (correct_i as f64 - i as f64).abs();
            distance += (correct_j as f64 - j as f64).abs();
        }
    }
    return distance;
}
