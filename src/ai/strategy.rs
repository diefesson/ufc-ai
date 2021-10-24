use std::collections::LinkedList;
use ordered_float::NotNan;
use priority_queue::PriorityQueue;

use super::node::*;

pub trait Strategy{
    fn add(&mut self, index: usize, node: &Node);

    fn update(&mut self, _index: usize, _new: &Node) {}

    fn next(&mut self) -> usize;

    fn len(&self) -> usize;
}

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

    fn len(&self) -> usize{
        self.stack.len()
    }
}

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

    fn len(&self) -> usize{
        self.queue.len()
    }
}

pub struct GreedyStrategy {
    queue: PriorityQueue<usize, NotNan<f64>>,
}

impl GreedyStrategy{
    pub fn new() -> Self{
        Self{
            queue: PriorityQueue::new()
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

    fn len(&self) -> usize{
        self.queue.len()
    }
}

pub struct AStarStrategy {
    queue: PriorityQueue<usize, NotNan<f64>>,
}

impl AStarStrategy{
    pub fn new() -> Self{
        Self{
            queue: PriorityQueue::new()
        }
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

    fn len(&self) -> usize{
        self.queue.len()
    }
}