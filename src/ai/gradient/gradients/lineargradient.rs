use crate::functions::multi_linear;
use std::iter::once;

fn m_gradient(ms: &[f64], c: f64, i: usize, xs: &Vec<Vec<f64>>, ys: &[f64]) -> f64 {
    let sum = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| x[i] * (y - multi_linear(ms, c, x)))
        .sum::<f64>();
    return (-2.0 / xs.len() as f64) * sum;
}

fn c_gradient(ms: &[f64], c: f64, xs: &Vec<Vec<f64>>, ys: &[f64]) -> f64 {
    let sum = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (y - multi_linear(ms, c, x)))
        .sum::<f64>();
    return (-2.0 / xs.len() as f64) * sum;
}

pub fn linear_gradient(params: &[f64], xs: &Vec<Vec<f64>>, ys: &[f64]) -> Vec<f64> {
    assert!(xs.len() == ys.len());
    let ms = &params[0..params.len() - 1];
    let c = params[params.len() - 1];
    (0..ms.len())
        .map(|i| m_gradient(ms, c, i, xs, ys))
        .chain(once(c_gradient(ms, c, xs, ys)))
        .collect()
}
