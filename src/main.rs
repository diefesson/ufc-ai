mod ai;
mod puzzle;

use ai::functions::*;
use ai::search::*;
use ai::strategy::*;

use puzzle::*;

fn main() {
    let ps = PuzzleState::<4>::new();
    let moved = ps.move_tile(LEFT).unwrap().move_tile(UP).unwrap();
    println!("{:?}", ps);
    println!("{}", wrong_place_heuristic(&moved));
    println!("{}", manhattan_heuristic(&moved));
}
