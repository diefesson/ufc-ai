use super::node::State;

pub type Verififer<S: State> = fn(&S) -> bool;
pub type Expander<S: State> = fn(&S) -> Vec<S>;
pub type Distance<S: State> = fn(&S, &S) -> f64;
pub type Heuristic<S: State> = fn(&S) -> f64;

pub fn one_distance<S: State>(_: &S) -> f64 {
    1.0
}

pub fn zero_heuristic<S: State>(_: &S) -> f64 {
    0.0
}
