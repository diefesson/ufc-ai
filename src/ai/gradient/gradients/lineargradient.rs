use crate::functions::linear;

fn m_gradient(m: f64, c: f64, xs: &[f64], ys: &[f64]) -> f64 {
    let sum = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| x * (y - linear(m, c, *x)))
        .sum::<f64>();
    return (-2.0 / xs.len() as f64) * sum;
}

fn c_gradient(m: f64, c: f64, xs: &[f64], ys: &[f64]) -> f64 {
    let sum = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (y - linear(m, c, *x)))
        .sum::<f64>();
    return (-2.0 / xs.len() as f64) * sum;
}

pub fn linear_gradient([m, c]: &[f64; 2], xs: &[f64], ys: &[f64]) -> [f64; 2] {
    [m_gradient(*m, *c, xs, ys), c_gradient(*m, *c, xs, ys)]
}
