use ordered_float::OrderedFloat;

pub fn nearest_index<const S: usize>(centers: &[[f64; S]], sample: &[f64; S]) -> usize {
    centers
        .iter()
        .enumerate()
        .min_by_key(|(_, c)| OrderedFloat(distance(c, sample)))
        .unwrap()
        .0
}

pub fn distance<const S: usize>(a: &[f64; S], b: &[f64; S]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn addi<const S: usize>(a: &mut [f64; S], b: &[f64; S]) {
    for i in 0..S {
        a[i] += b[i];
    }
}

pub fn divi<const S: usize>(a: &mut [f64; S], b: f64) {
    for i in 0..S {
        a[i] /= b;
    }
}
