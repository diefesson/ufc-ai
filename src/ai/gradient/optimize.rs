pub fn optimize<G, O>(
    initial: Vec<f64>,
    rate: f64,
    epochs: usize,
    gradient: G,
    mut on_epoch: O,
) -> Vec<f64>
where
    G: Fn(&[f64]) -> Vec<f64>,
    O: FnMut(usize, &Vec<f64>),
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
