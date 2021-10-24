mod ai;
mod puzzle;

use ai::functions::*;
use ai::search::*;
use ai::strategy::*;

use puzzle::*;

const PUZZLE: [[i32; 4];4] = [
    [1, 2, 3, 4],
    [5, 6, 8, 12],
    [13, 9, 16, 7],
    [14, 11, 10, 15]
];

fn main() {
    let ps = PuzzleState::<4>::new_with_numbers(PUZZLE);
    let path = search(
        &ps,
        true,
        AStarStrategy::new(),
        puzzle_verifier,
        puzzle_expander,
        one_distance,
        manhattan_heuristic,
    );
    for p in path{
        println!("{:?}, {:?}", p.node(), p.state());
    }
}
