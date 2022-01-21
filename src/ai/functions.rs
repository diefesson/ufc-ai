use core::f64;

use crate::ai::search::data::State;

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
    assert_eq!(xs.len(), ys.len(), "xs and ys lens not matches");
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| (y - function(x)).powi(2))
        .sum()
}

pub fn classification_accuracy<F, X, Y>(function: F, xs: &[X], ys: &[Y]) -> f64
where
    F: Fn(&X) -> Y,
    Y: Eq,
{
    assert_eq!(xs.len(), ys.len(), "xs and ys lens not matches");
    let correct = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| function(x) == *y)
        .filter(|&b| b)
        .count();
    correct as f64 / xs.len() as f64
}

pub fn multi_linear<const S: usize>(c: f64, ms: &[f64; S], x: &[f64; S]) -> f64 {
    assert!(ms.len() == x.len());
    ms.iter().zip(x.iter()).map(|(m, x)| m * x).sum::<f64>() + c
}

pub fn linear(c: f64, m: f64, x: f64) -> f64 {
    multi_linear(c, &[m], &[x])
}
