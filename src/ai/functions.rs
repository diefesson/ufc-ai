use super::node::State;

// Rust yet doesnt support enforcing generics on type aliases,
// but its being guaranted in the dependents of these aliases
// so the warning can be safely ignored.
#[allow(type_alias_bounds)]
pub type Verififer<S: State> = fn(&S) -> bool;
#[allow(type_alias_bounds)]
pub type Expander<S: State> = fn(&S) -> Vec<S>;
#[allow(type_alias_bounds)]
pub type Distance<S: State> = fn(&S, &S) -> f64;
#[allow(type_alias_bounds)]
pub type Heuristic<S: State> = fn(&S) -> f64;

pub fn one_distance<S: State>(_:&S, _: &S) -> f64 {
    1.0
}

pub fn zero_heuristic<S: State>(_: &S) -> f64 {
    0.0
}
