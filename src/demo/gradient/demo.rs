use crate::demo::gradient::{DATASET_PATH, OUTPUT_PATH};
use crate::functions::{linear, mse};
use crate::gradient::gradients::linear_gradient;
use crate::gradient::optimize;
use csv::{Reader, Writer};
use rand::prelude::StdRng;
use rand::seq::index::sample;
use rand::SeedableRng;
use std::error::Error;

pub fn demo() -> Result<(), Box<dyn Error>> {
    let mut random = StdRng::seed_from_u64(2334234);
    let (x_data, y_data): (Vec<_>, Vec<_>) = Reader::from_path(DATASET_PATH)?
        .deserialize()
        .map(|r: Result<(f64, f64), _>| {
            let r = r.unwrap();
            (vec![r.0], r.1)
        })
        .unzip();

    let subset_size = (x_data.len() as f64 * 0.2) as usize;
    let subset_indexes = sample(&mut random, x_data.len(), subset_size);
    let x_subset: Vec<_> = subset_indexes.iter().map(|i| x_data[i].clone()).collect();
    let y_subset: Vec<_> = subset_indexes.iter().map(|i| y_data[i]).collect();

    let mut mses = Vec::<f64>::new();

    let params = optimize(
        [1.0, 0.0].into(),
        0.001,
        1000,
        |params| linear_gradient(params, &x_subset, &y_subset),
        |e, params| {
            let m = params[0];
            let c = params[1];
            let error = mse(|x| linear(m, c, x[0]), &x_data, &y_data);
            println!("completed epoch {}, m: {} c: {}, mse: {}", e, m, c, error);

            mses.push(error);
        },
    );

    println!("finished, m: {} c: {}", params[0], params[1]);

    let mut writer = Writer::from_path(OUTPUT_PATH)?;
    for record in mses.iter().enumerate() {
        writer.serialize(record)?;
    }

    Ok(())
}
