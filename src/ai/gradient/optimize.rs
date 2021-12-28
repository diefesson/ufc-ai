pub fn optimize<G, O, const S: usize>(
    initial: [f64; S],
    rate: f64,
    epochs: usize,
    gradient: G,
    mut on_epoch: O,
) -> [f64; S]
where
    G: Fn(&[f64; S]) -> [f64; S],
    O: FnMut(usize, &[f64; S]),
{
    let mut value = initial;
    for e in 0..epochs {
        let direction = gradient(&value);
        for i in 0..value.len() {
            value[i] -= rate * direction[i];
        }
        on_epoch(e, &value);
    }
    return value;
}
