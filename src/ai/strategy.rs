use ordered_float::NotNan;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::collections::LinkedList;

use super::node::*;

pub trait Strategy {
    fn add(&mut self, index: usize, node: &Node);

    fn update(&mut self, _index: usize, _old: &Node, _new: &Node) {}

    fn next(&mut self) -> usize;
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
}

/* pub struct FifoStrategy<'a, S: State> {
    queue: LinkedList<&'a S>,
}

impl<'a, S: State> Strategy<'a, S> for FifoStrategy<'a, S> {
    fn new() -> Self {
        Self {
            queue: LinkedList::new(),
        }
    }

    fn add(&mut self, state: &'a S, _: &'a Node) {
        self.queue.push_back(state);
    }

    fn next(&mut self) -> &'a S {
        self.queue.pop_front().unwrap()
    }
}

pub struct GreedyStrategy<'a, S: State> {
    queue: PriorityQueue<&'a S, NotNan<f64>>,
}

impl<'a, S: State> Strategy<'a, S> for GreedyStrategy<'a, S> {
    fn new() -> Self {
        Self {
            queue: PriorityQueue::new(),
        }
    }

    fn add(&mut self, state: &'a S, node: &'a Node) {
        self.queue
            .push(state, NotNan::new(-node.heuristic).unwrap());
    }

    fn next(&mut self) -> &'a S {
        self.queue.pop().unwrap().0
    }
} */

/* pub struct AStarStrategy<'a, S: State> {
    queue: PriorityQueue<&'a Node<S>, NotNan<f64>>,
}

impl<'a, S: State> Strategy<'a, S> for AStarStrategy<'a, S> {
    fn add(&mut self, node: &'a Node<S>) {
        self.queue.push(node, NotNan::new(-node.total()).unwrap());
    }

    fn update(&mut self, node: &'a Node<S>) {
        self.queue
            .push_increase(node, NotNan::new(-node.total()).unwrap());
    }

    fn next(&mut self) -> &'a Node<S> {
        self.queue.pop().unwrap().0
    }
}
 */
