use std::{collections::HashMap, hash::Hash};

pub fn encode_labels<T: Eq + Hash + Clone>(original: &[T]) -> (Vec<u64>, HashMap<T, u64>) {
    let mut index = 0;
    let mut mapping = HashMap::new();
    let labels = original
        .iter()
        .map(|o| {
            if mapping.contains_key(o) {
                mapping[o]
            } else {
                mapping.insert(o.clone(), index);
                index += 1;
                index - 1
            }
        })
        .collect();
    (labels, mapping)
}
