mod ai;
mod puzzle;

use ai::functions::*;
use ai::search::*;
use ai::strategy::*;

use puzzle::*;

fn main() {
    let ps = PuzzleState::<4>::new();
    let moved = ps.move_tile(Left).unwrap().move_tile(Up).unwrap();
    let path = search(
        &moved,
        GreedyStrategy::new(),
        puzzle_verifier,
        puzzle_expander,
        one_distance,
        wrong_place_heuristic,
    );
    for p in path{
        println!("{:?}, {:?}", p.node(), p.state());
    }
}
