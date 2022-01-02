pub fn optimize<G, R, O, const S: usize>(
    mut value: [f64; S],
    epochs: usize,
    rate: f64,
    lambda: [f64; S],
    gradient: G,
    regularization: R,
    mut on_epoch: O,
) -> [f64; S]
where
    G: Fn(&[f64; S]) -> [f64; S],
    R: Fn(&[f64; S]) -> [f64; S],
    O: FnMut(usize, &[f64; S]),
{
    for e in 0..epochs {
        let g = gradient(&value);
        let r = regularization(&value);
        for i in 0..value.len() {
            value[i] -= rate * (g[i] + lambda[i] * r[i]);
        }
        on_epoch(e, &value);
    }
    value
}
