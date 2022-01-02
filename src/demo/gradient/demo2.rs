use crate::functions::{mse, multi_linear};
use crate::gradient::regularizations::no_regularization;
use crate::gradient::{gradients::new_linear_gradient, optimize};
use csv::{ReaderBuilder, Writer};
use rand::{prelude::StdRng, seq::index::sample, SeedableRng};
use std::convert::TryInto;
use std::error::Error;

const DATASET_PATH: &str = "data/ex1data2.txt";
const OUTPUT_PATH: &str = "output/output-2.csv";

pub fn demo_2() -> Result<(), Box<dyn Error>> {
    let mut random = StdRng::seed_from_u64(12345678);
    let mut reader = ReaderBuilder::new().from_path(DATASET_PATH)?;
    let (x_data, y_data): (Vec<_>, Vec<_>) = reader
        .deserialize()
        .map(|r: Result<(f64, f64, f64), _>| {
            let r = r.unwrap();
            ([r.0, r.1], r.2)
        })
        .unzip();

    let subset_size = (x_data.len() as f64 * 0.2) as usize;
    let subset_indexes = sample(&mut random, x_data.len(), subset_size);
    let x_subset: Vec<_> = subset_indexes.iter().map(|i| x_data[i].clone()).collect();
    let y_subset: Vec<_> = subset_indexes.iter().map(|i| y_data[i]).collect();

    let mut mses = Vec::<f64>::new();

    let params = optimize(
        [1.0, 1.0, 0.0].into(),
        100,
        0.01,
        Default::default(),
        new_linear_gradient(&x_subset, &y_subset),
        no_regularization,
        |e, params| {
            let c = params[0];
            let ms = params[1..].try_into().unwrap();
            let error = mse(|x| multi_linear(c, ms, x), &x_data, &y_data);
            mses.push(error);
            println!("completed epoch {}, params: {:?}, mse: {}", e, ms, error);
        },
    );

    println!("finished, params: {:?}", params);

    let mut writer = Writer::from_path(OUTPUT_PATH)?;
    for record in mses.iter().enumerate() {
        writer.serialize(record)?;
    }

    Ok(())
}
