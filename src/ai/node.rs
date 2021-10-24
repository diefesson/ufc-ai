use std::hash::{Hash};

pub trait State: Eq + Clone + Hash {}

// TODO: update data access
#[derive(Clone, Default)]
pub struct Node {
    pub distance: f64,
    pub heuristic: f64,
}

impl Node {
    pub fn total(&self) -> f64 {
        self.distance + self.heuristic
    }
}
