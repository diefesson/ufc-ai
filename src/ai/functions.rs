use core::f64;

use crate::ai::data::State;

pub fn one_distance<S: State>(_: &S, _: &S) -> f64 {
    1.0
}

pub fn zero_heuristic<S: State>(_: &S) -> f64 {
    0.0
}

pub fn mse<F, T>(function: F, xs: &[T], ys: &[f64]) -> f64
where
    F: Fn(&T) -> f64,
{
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| (y - function(x)).powi(2))
        .sum()
}

pub fn multi_linear<const S: usize>(ms: &[f64; S], c: f64, x: &[f64; S]) -> f64 {
    ms.iter().zip(x.iter()).map(|(m, x)| m * x).sum::<f64>() + c
}

pub fn linear(m: f64, c: f64, x: f64) -> f64 {
    multi_linear(&[m], c, &[x])
}
