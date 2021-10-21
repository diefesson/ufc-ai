use super::functions::{Distance, Expander, Heuristic};
use super::node::{Node, State};

pub fn expand_node<S: State>(
    node: &Node<S>,
    expander: Expander<S>,
    distance: Distance<S>,
    heuristic: Heuristic<S>,
) -> Vec<Node<S>> {
    expander(&node.state())
        .into_iter()
        .map(|s| {
            Node::new_with_distance_and_heuristic(
                s.clone(),
                distance(&node.state(), &s),
                heuristic(&s),
            )
        })
        .collect()
}
