use std::collections::LinkedList;

use crate::ai::data::Node;
use crate::ai::search::strategy::Strategy;

pub struct FifoStrategy {
    queue: LinkedList<usize>,
}

impl FifoStrategy {
    pub fn new() -> Self {
        Self {
            queue: LinkedList::new(),
        }
    }
}

impl Strategy for FifoStrategy {
    fn add(&mut self, index: usize, _: &Node) {
        self.queue.push_back(index);
    }

    fn next(&mut self) -> usize {
        self.queue.pop_front().unwrap()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}
