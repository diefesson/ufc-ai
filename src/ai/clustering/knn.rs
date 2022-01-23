use super::ops::distance;
use crate::utilities::OrdKeyPair;
use ordered_float::OrderedFloat;
use std::collections::{BinaryHeap, HashMap};

type Entry = OrdKeyPair<OrderedFloat<f64>, u64>;

pub fn knn<const S: usize>(
    x_data: &[[f64; S]],
    y_data: &[u64],
    neighbor_count: usize,
    sample: &[f64; S],
) -> u64 {
    let nearests = find_k_nearests(x_data, y_data, neighbor_count, sample);
    most_repeated(&nearests)
}

fn find_k_nearests<const S: usize>(
    x_data: &[[f64; S]],
    y_data: &[u64],
    k: usize,
    sample: &[f64; S],
) -> Vec<u64> {
    assert_eq!(
        x_data.len(),
        y_data.len(),
        "x_data and y_data lens not matches"
    );
    let mut nearests = BinaryHeap::new();
    for (x, y) in x_data.iter().zip(y_data.iter()) {
        let distance = distance(sample, x);
        nearests.push(Entry::new(OrderedFloat(distance), *y));
        if nearests.len() > k {
            nearests.pop();
        }
    }
    nearests.iter().map(|e| e.1).collect()
}

fn most_repeated(elements: &[u64]) -> u64 {
    let mut count = HashMap::new();
    for e in elements {
        match count.get_mut(e) {
            Some(c) => {
                *c += 1;
            }
            None => {
                count.insert(*e, 1);
            }
        }
    }
    count.into_iter().max_by_key(|(_, c)| *c).unwrap().0
}
