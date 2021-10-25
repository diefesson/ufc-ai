use crate::ai::data::Node;
use crate::ai::search::strategy::Strategy;

pub struct LifoStrategy {
    stack: Vec<usize>,
}

impl LifoStrategy {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

impl Strategy for LifoStrategy {
    fn add(&mut self, index: usize, _: &Node) {
        self.stack.push(index);
    }

    fn next(&mut self) -> usize {
        self.stack.pop().unwrap()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }
}
