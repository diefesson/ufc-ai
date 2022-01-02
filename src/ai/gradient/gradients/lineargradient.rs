use std::convert::TryInto;

use crate::functions::multi_linear;

fn m_gradient<const S: usize>(c: f64, ms: &[f64; S], i: usize, xs: &[[f64; S]], ys: &[f64]) -> f64 {
    let sum = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| x[i] * (y - multi_linear(c, ms, x)))
        .sum::<f64>();
    return (-2.0 / xs.len() as f64) * sum;
}

fn c_gradient<const S: usize>(c: f64, ms: &[f64; S], xs: &[[f64; S]], ys: &[f64]) -> f64 {
    let sum = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (y - multi_linear(c, ms, x)))
        .sum::<f64>();
    return (-2.0 / xs.len() as f64) * sum;
}

pub fn linear_gradient<const S: usize, const O: usize>(
    c: f64,
    ms: &[f64; S],
    xs: &[[f64; S]],
    ys: &[f64],
) -> [f64; O] {
    assert!(xs.len() == ys.len());
    let mut gradient = [0.0; O];
    gradient[0] = c_gradient(c, ms, xs, ys);
    for i in 0..ms.len() {
        gradient[i + 1] = m_gradient(c, ms, i, xs, ys);
    }
    return gradient;
}

pub fn new_linear_gradient<'c, const S: usize, const I: usize>(
    xs: &'c [[f64; I]],
    ys: &'c [f64],
) -> impl Fn(&[f64; S]) -> [f64; S] + 'c {
    debug_assert!(
        S == I + 1,
        "params should be 1 unit greater than x, because c"
    );
    move |params| linear_gradient(params[0], params[1..].try_into().unwrap(), xs, ys)
}
