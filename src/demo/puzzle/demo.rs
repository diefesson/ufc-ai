use std::time::Instant;

use crate::ai::functions::*;
use crate::ai::search::search;
use crate::ai::search::strategy::*;
use crate::demo::puzzle::*;

const PUZZLE: [[i32; 4]; 4] = [
    [1, 2, 3, 4],
    [5, 6, 8, 12],
    [13, 9, 16, 7],
    [14, 11, 10, 15],
];

#[allow(dead_code)]
pub fn demo() {
    let ps = PuzzleState::with_numbers(PUZZLE);

    let start_time = Instant::now();
    let path = search(
        ps,
        true,
        AStarStrategy::new(),
        &puzzle_expander,
        &one_distance,
        &manhattan_heuristic,
        &puzzle_verifier,
    );
    let end_time = Instant::now();

    let duration = end_time - start_time;
    let n_steps = path.len() - 1;
    println!(
        "Solução com {:?} passos encontrada em {:?} segundos",
        n_steps, duration
    );
}
