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
    update: bool,
    mut strategy: impl Strategy,
    verifier: Verififer<S>,
    expander: Expander<S>,
    distance: Distance<S>,
    heuristic: Heuristic<S>,
) -> Vec<PathEntry<S>> {
    let first_entry = SearchEntry {
        state: start.clone(),
        node: Node {
            distance: 0.0,
            heuristic: heuristic(start),
        },
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
                update,
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
    update: bool,
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
    for i in 0..next_indexes.len() {
        if next_indexes[i].is_none() {
            let new_index = know.len();
            next_indexes[i] = Some(new_index);
            strategy.add(new_index, &next_nodes[i]);
            know.push(SearchEntry {
                state: next_states[i].clone(),
                node: next_nodes[i].clone(),
                previous: Some(index),
                successors: None,
            });
        } else {
            let know_index = next_indexes[i].unwrap();
            let old = &know[know_index].node;
            let new = &next_nodes[i];
            if update && new.distance < old.distance {
                let distance_reduction = new.distance - old.distance;
                know[know_index].previous = Some(i);
                reduce_distance(know_index, distance_reduction, know, strategy);
            }
        }
    }
    know[index].successors = Some(next_indexes.into_iter().map(|i| i.unwrap()).collect());
}

fn reduce_distance<S: State>(
    index: usize,
    reduction: f64,
    know: &mut Vec<SearchEntry<S>>,
    strategy: &mut impl Strategy,
) {
    know[index].node.distance -= reduction;
    let successors = know[index].successors.clone();
    match successors {
        Some(successors) => {
            for s in successors {
                if know[s].previous == Some(index) {
                    reduce_distance(s, reduction, know, strategy);
                }
            }
        }
        None => strategy.update(index, &know[index].node),
    }
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
