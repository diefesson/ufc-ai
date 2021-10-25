use super::node::State;

pub fn one_distance<S: State>(_:&S, _: &S) -> f64 {
    1.0
}

pub fn zero_heuristic<S: State>(_: &S) -> f64 {
    0.0
}
