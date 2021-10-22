use super::functions::*;
use super::node::*;
use super::strategy::Strategy;

struct SearchEntry<S: State> {
    state: S,
    node: Node,
    previous: Option<usize>,
    successors: Option<Vec<usize>>,
}

pub fn search<S: State>(
    start: &S,
    mut strategy: impl Strategy,
    verifier: Verififer<S>,
    expander: Expander<S>,
    distance: Distance<S>,
    heuristic: Heuristic<S>,
) {
    let mut know = Vec::new();
    let start_node = Default::default();
    strategy.add(0, &start_node);
    know.push(SearchEntry {
        state: start.clone(),
        node: start_node,
        previous: None,
        successors: None,
    });
    loop {
        let index = strategy.next();
        let search_entry = know.get(index).unwrap();
        if verifier(&search_entry.state) {
            // TODO: return logic
            println!("found");
            break;
        }
        let next_states = expander(&search_entry.state);
        let mut next_indexes = next_states
            .iter()
            .map(|s| know.iter().position(|se| se.state == *s))
            .collect::<Vec<_>>();
        let next_nodes = next_states
            .iter()
            .map(|s| Node {
                distance: search_entry.node.distance + distance(&search_entry.state, s),
                heuristic: heuristic(s),
            })
            .collect::<Vec<_>>();
        for j in 0..next_indexes.len() {
            if next_indexes[j].is_none() {
                let new_index = know.len();
                next_indexes[j] = Some(new_index);
                strategy.add(new_index, &next_nodes[j]);
                know.push(SearchEntry {
                    state: next_states[j].clone(),
                    node: next_nodes[j].clone(),
                    previous: Some(index),
                    successors: None,
                })
            } else {
                let know_index = next_indexes[j].unwrap();
                let _better = strategy.update(know_index, &know[know_index].node, &next_nodes[j]);
                // TODO: distance update logic
            }
        }
        know[index].successors = Some(next_indexes.into_iter().map(|i| i.unwrap()).collect());
    }
}
