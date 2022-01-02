use ordered_float::NotNan;
use priority_queue::PriorityQueue;

use crate::ai::data::Node;
use crate::ai::search::strategies::Strategy;

pub struct AStarStrategy {
    queue: PriorityQueue<usize, NotNan<f64>>,
}

impl AStarStrategy {
    pub fn new() -> Self {
        Self {
            queue: PriorityQueue::new(),
        }
    }
}

impl Default for AStarStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for AStarStrategy {
    fn add(&mut self, index: usize, node: &Node) {
        self.queue.push(index, NotNan::new(-node.total()).unwrap());
    }

    fn update(&mut self, index: usize, new: &Node) {
        self.queue
            .push_increase(index, NotNan::new(-new.total()).unwrap());
    }

    fn next(&mut self) -> usize {
        self.queue.pop().unwrap().0
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
