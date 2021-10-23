mod ai;
mod puzzle;

use ai::functions::*;
use ai::search::*;
use ai::strategy::*;

use puzzle::*;

fn main() {
    let ps = PuzzleState::<4>::new();
    let moved = ps.move_tile(LEFT).unwrap();
    println!("{:?}", ps);
    println!("{}", ps.inversion_count());
}
