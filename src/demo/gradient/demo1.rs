use crate::functions::{linear, mse};
use crate::gradient::gradients::new_linear_gradient;
use crate::gradient::optimize;
use crate::gradient::regularizations::no_regularization;
use csv::{Reader, Writer};
use rand::prelude::StdRng;
use rand::seq::index::sample;
use rand::SeedableRng;
use std::error::Error;

const DATASET_PATH_1: &str = "data/ex1data1.txt";
const OUTPUT_PATH_1: &str = "output/output-1.csv";

pub fn demo_1() -> Result<(), Box<dyn Error>> {
    let mut random = StdRng::seed_from_u64(2334234);
    let mut x_data = Vec::<[f64; 1]>::new();
    let mut y_data = Vec::<f64>::new();
    for r in Reader::from_path(DATASET_PATH_1)?.deserialize() {
        let (x, y) = r?;
        x_data.push([x]);
        y_data.push(y);
    }

    let subset_size = (x_data.len() as f64 * 0.2) as usize;
    let subset_indexes = sample(&mut random, x_data.len(), subset_size);
    let x_subset: Vec<_> = subset_indexes.iter().map(|i| x_data[i].clone()).collect();
    let y_subset: Vec<_> = subset_indexes.iter().map(|i| y_data[i]).collect();

    let mut mses = Vec::<f64>::new();

    let params = optimize(
        [0.0, 1.0],
        1000,
        0.001,
        Default::default(),
        new_linear_gradient(&x_subset, &y_subset),
        no_regularization,
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
