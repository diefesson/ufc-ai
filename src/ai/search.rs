use super::functions::*;
use super::node::*;
use super::strategy::Strategy;

struct SearchEntry<S: State> {
    state: S,
    node: Node,
    previous: Option<usize>,
    successors: Option<Vec<usize>>,
}

impl<S: State> SearchEntry<S> {
    fn to_path_entry(&self) -> PathEntry<S> {
        PathEntry {
            state: self.state.clone(),
            node: self.node.clone(),
        }
    }
}

pub struct PathEntry<S: State> {
    state: S,
    node: Node,
}

impl<S: State> PathEntry<S> {
    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn node(&self) -> &Node {
        &self.node
    }
}

pub fn search<S: State>(
    start: &S,
    mut strategy: impl Strategy,
    verifier: Verififer<S>,
    expander: Expander<S>,
    distance: Distance<S>,
    heuristic: Heuristic<S>,
) -> Vec<PathEntry<S>> {
    let first_entry = SearchEntry {
        state: start.clone(),
        node: Default::default(),
        previous: None,
        successors: None,
    };
    let mut know = Vec::new();
    strategy.add(0, &first_entry.node);
    know.push(first_entry);
    loop {
        let index = strategy.next();
        let search_entry = know.get(index).unwrap();
        if verifier(&search_entry.state) {
            return build_path(index, know);
        } else {
            expand(
                index,
                &mut know,
                &mut strategy,
                &expander,
                &distance,
                &heuristic,
            );
        }
    }
}

fn expand<S: State>(
    index: usize,
    know: &mut Vec<SearchEntry<S>>,
    strategy: &mut impl Strategy,
    expander: &Expander<S>,
    distance: &Distance<S>,
    heuristic: &Heuristic<S>,
) {
    let search_entry = &know[index];
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
            });
        } else {
            let know_index = next_indexes[j].unwrap();
            let _better = strategy.update(know_index, &know[know_index].node, &next_nodes[j]);
            // TODO: distance update logic
        }
    }
    know[index].successors = Some(next_indexes.into_iter().map(|i| i.unwrap()).collect());
}

fn build_path<S: State>(end: usize, know: Vec<SearchEntry<S>>) -> Vec<PathEntry<S>> {
    let mut path = Vec::new();
    let mut i = end;
    loop {
        let se = &know[i];
        path.push(se.to_path_entry());
        if se.previous.is_some() {
            i = se.previous.unwrap()
        } else {
            break;
        }
    }
    return path;
}
