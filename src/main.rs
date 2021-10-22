mod ai;
mod puzzle;

use ai::functions::*;
use ai::search::*;
use ai::strategy::*;

use puzzle::*;

fn main() {
    let s = PuzzleState::new();
    println!("{:?}", s);
    println!("{:?}", s.move_tile(LEFT).unwrap());
}
