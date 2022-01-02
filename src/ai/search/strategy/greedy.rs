use ordered_float::NotNan;
use priority_queue::PriorityQueue;

use crate::ai::data::Node;
use crate::ai::search::strategy::Strategy;

pub struct GreedyStrategy {
    queue: PriorityQueue<usize, NotNan<f64>>,
}

impl GreedyStrategy {
    pub fn new() -> Self {
        Self {
            queue: PriorityQueue::new(),
        }
    }
}

impl Strategy for GreedyStrategy {
    fn add(&mut self, index: usize, node: &Node) {
        self.queue
            .push(index, NotNan::new(-node.heuristic).unwrap());
    }

    fn next(&mut self) -> usize {
        self.queue.pop().unwrap().0
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}
