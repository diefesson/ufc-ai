use crate::ai::search::data::Node;

pub trait Strategy {
    fn add(&mut self, index: usize, node: &Node);

    fn update(&mut self, _index: usize, _new: &Node) {}

    fn next(&mut self) -> usize;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}
