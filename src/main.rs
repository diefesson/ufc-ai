mod ai;

use ai::functions::*;
use ai::search::*;
use ai::strategy::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct NumberState(i32);

impl ai::node::State for NumberState{}

fn expander(ns: &NumberState) -> Vec<NumberState>{
    vec![NumberState(ns.0 - 1), NumberState(ns.0 + 1)]
}

fn main() {
    let n = NumberState(5);
    let path = search(&n, FifoStrategy::new(), |n| n.0 == 10, expander, one_distance, zero_heuristic);
    for s in path{
        println!("{:?}", s.state());
    }
    println!("Hello");
}
