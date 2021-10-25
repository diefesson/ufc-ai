use std::hash::Hash;

pub trait State: Eq + Clone + Hash {}
