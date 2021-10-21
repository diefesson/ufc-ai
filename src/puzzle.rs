use super::ai;

use ai::node::State;

#[derive(PartialEq, Eq, Clone, Hash)]
struct PuzzleState {
    numbers: [[i32; 4]; 4],
}

fn puzzle_state_expander(state: PuzzleState) -> Vec<PuzzleState> {
    let nexts = 
}
