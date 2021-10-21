use std::collections::HashSet;

use super::functions::*;
use super::node::*;
use super::util::*;

pub struct DepthSearch<S: State> {
    start: Node<S>,
    path: Vec<Node<S>>,
    know: HashSet<Node<S>>,
    verifier: Verififer<S>,
    expander: Expander<S>,
    distance: Distance<S>,
}

impl<S: State> DepthSearch<S> {
    fn new(start: S, verifier: Verififer<S>, expander: Expander<S>, distance: Distance<S>) -> Self {
        Self {
            start: Node::new(start),
            path: Vec::new(),
            know: HashSet::new(),
            verifier,
            expander,
            distance,
        }
    }

    pub fn run(mut self) -> Vec<Node<S>> {
        self.search(&self.start.clone());
        return self.path;
    }

    fn search(&mut self, node: &Node<S>) -> bool {
        self.path.push(node.clone());
        if (self.verifier)(&node.state()) {
            return true;
        } else {
            for n in expand_node(node, self.expander, self.distance, zero_heuristic) {
                if !self.know.contains(&n) {
                    self.know.insert(n.clone());
                    if self.search(&n) {
                        return true;
                    }
                }
            }
        }
        self.path.pop();
        return false;
    }
}
