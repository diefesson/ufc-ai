use std::hash::{Hash, Hasher};

pub trait State: Eq + Clone + Hash {}

#[derive(Clone)]
pub struct Node<S: State> {
    state: S,
    distance: f64,
    heuristic: f64,
}

impl<S: State> Node<S> {
    pub fn new(state: S) -> Self {
        Self::new_with_distance(state, 0.0)
    }

    pub fn new_with_distance(state: S, distance: f64) -> Self {
        Self::new_with_distance_and_heuristic(state, distance, 0.0)
    }

    pub fn new_with_distance_and_heuristic(state: S, distance: f64, heuristic: f64) -> Self {
        Self {
            state,
            distance,
            heuristic,
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn heuristic(&self) -> f64 {
        self.heuristic
    }

    pub fn total(&self) -> f64 {
        self.distance + self.heuristic
    }
}

impl<S: State> PartialEq for Node<S> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<S: State> Eq for Node<S> {}

impl<S: State> Hash for Node<S> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.state.hash(hasher);
    }
}
