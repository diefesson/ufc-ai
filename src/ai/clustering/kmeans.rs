use super::ops::{addi, divi, nearest_index};

pub fn k_means<const S: usize>(
    mut centers: Vec<[f64; S]>,
    samples: &[[f64; S]],
    epochs: usize,
) -> Vec<[f64; S]> {
    let cluster_count = centers.len();
    for _ in 0..epochs {
        let mut next_centers = vec![[0.; S]; centers.len()];
        let mut cluster_sizes = vec![0; S];
        for s in samples {
            let cluster_index = nearest_index(&centers, s);
            let sum = &mut next_centers[cluster_index];
            addi(sum, s);
            cluster_sizes[cluster_index] += 1;
        }
        for i in 0..cluster_count {
            divi(&mut next_centers[i], cluster_sizes[i] as f64);
        }
        if centers == next_centers {
            break;
        }
        centers = next_centers;
    }
    centers
}
