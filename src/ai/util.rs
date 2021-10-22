use super::functions::{Distance, Expander, Heuristic};
use super::node::{Node, State};

pub fn expand_node<S: State>(
    state: &State,
    node: &Node,
    expander: Expander<S>,
    distance: Distance<S>,
    heuristic: Heuristic<S>,
) -> Vec<Node<S>> {
    let next_states = expander(&state)
    let next_nodes = next_states.iter().map(|s|{
        Node{
            previous
        }
    })
    expander(&state)
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
