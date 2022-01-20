use super::ops::distance;
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Entry(OrderedFloat<f64>, usize);

pub fn knn<const S: usize>(
    clusters: Vec<Vec<[f64; S]>>,
    neighbor_count: usize,
    sample: &[f64; S],
) -> usize {
    let mut nearests = BinaryHeap::new();
    for (i, cluster) in clusters.iter().enumerate() {
        for vector in cluster {
            let distance = distance(sample, &vector);
            nearests.push(Entry(OrderedFloat(distance), i));
        }
        if nearests.len() > neighbor_count {
            nearests.pop();
        }
    }
    let mut counts = vec![0; clusters.len()];
    nearests.iter().for_each(|e| counts[e.1] += 1);
    counts.iter().enumerate().max_by_key(|(_, c)| *c).unwrap().0
}
