use crate::demo::gradient::{DATASET_PATH_1, OUTPUT_PATH_1};
use crate::functions::{linear, mse};
use crate::gradient::gradients::linear_gradient;
use crate::gradient::optimize;
use csv::{Reader, Writer};
use rand::prelude::StdRng;
use rand::seq::index::sample;
use rand::SeedableRng;
use std::convert::TryInto;
use std::error::Error;

pub fn demo_1() -> Result<(), Box<dyn Error>> {
    let mut random = StdRng::seed_from_u64(2334234);
    let (x_data, y_data): (Vec<_>, Vec<_>) = Reader::from_path(DATASET_PATH_1)?
        .deserialize()
        .map(|r: Result<(f64, f64), _>| {
            let r = r.unwrap();
            ([r.0], r.1)
        })
        .unzip();

    let subset_size = (x_data.len() as f64 * 0.2) as usize;
    let subset_indexes = sample(&mut random, x_data.len(), subset_size);
    let x_subset: Vec<_> = subset_indexes.iter().map(|i| x_data[i].clone()).collect();
    let y_subset: Vec<_> = subset_indexes.iter().map(|i| y_data[i]).collect();

    let mut mses = Vec::<f64>::new();

    let params = optimize(
        [0.0, 1.0],
        0.001,
        1000,
        |params| {
            linear_gradient(
                params[0],
                params[1..].try_into().unwrap(),
                &x_subset,
                &y_subset,
            )
        },
        |e, params| {
            let c = params[0];
            let m = params[1];
            let error = mse(|x| linear(c, m, x[0]), &x_data, &y_data);
            mses.push(error);
            println!("epoch {}, m: {} c: {}, mse: {}", e, m, c, error);
        },
    );

    println!("finished, m: {} c: {}", params[1], params[0]);

    let mut writer = Writer::from_path(OUTPUT_PATH_1)?;
    for record in mses.iter().enumerate() {
        writer.serialize(record)?;
    }

    Ok(())
}
