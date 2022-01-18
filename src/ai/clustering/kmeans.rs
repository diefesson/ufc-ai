use ordered_float::OrderedFloat;
use std::iter::Iterator;

pub fn k_means<const S: usize>(
    mut clusters: Vec<[f64; S]>,
    samples: &[[f64; S]],
    epochs: usize,
) -> Vec<[f64; S]> {
    let cluster_count = clusters.len();
    for _ in 0..epochs {
        let mut cluster_sums = vec![[0.; S]; clusters.len()];
        let mut cluster_sizes = vec![0; S];
        for s in samples {
            let cluster_index = nearest_index(&clusters, s);
            let sum = &mut cluster_sums[cluster_index];
            for i in 0..S {
                sum[i] += s[i];
            }
            cluster_sizes[cluster_index] += 1;
        }
        for i in 0..cluster_count {
            clusters[i] = cluster_sums[i].map(|c| c / cluster_sizes[i] as f64);
        }
    }
    clusters
}

fn nearest_index<const S: usize>(clusters: &[[f64; S]], sample: &[f64; S]) -> usize {
    clusters
        .iter()
        .enumerate()
        .min_by_key(|(_, c)| OrderedFloat(distance(c, sample)))
        .unwrap()
        .0
}

fn distance<const S: usize>(a: &[f64; S], b: &[f64; S]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}
