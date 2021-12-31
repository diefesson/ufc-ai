pub fn l1<const S: usize>(params: &[f64; S]) -> [f64; S] {
    params.map(|p| p.signum())
}

pub fn l2<const S: usize>(params: &[f64; S]) -> [f64; S] {
    params.map(|p| 2. * p)
}

pub fn no_regularization<const S: usize>(_params: &[f64; S]) -> [f64; S] {
    [0.0; S]
}
