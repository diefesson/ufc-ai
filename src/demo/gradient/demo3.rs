use crate::functions::{mse, multi_linear};
use crate::gradient::gradients::new_linear_gradient;
use crate::gradient::optimize;
use crate::gradient::regularizations::l1;
use csv::{Reader, Writer};
use serde::Serialize;
use std::convert::TryInto;
use std::error::Error;

const DATASET_PATH: &str = "data/ex1data3.txt";
const TRAIN_OUTPUT_PATH: &str = "output/output-3-train.csv";
const TEST_OUTPUT_PATH: &str = "output/output-3-test.csv";

pub fn demo_3() -> Result<(), Box<dyn Error>> {
    let mut x_data = vec![];
    let mut y_data = vec![];
    for r in Reader::from_path(DATASET_PATH)?.deserialize() {
        let r: [f64; 6] = r?;
        x_data.push([r[0], r[1], r[2], r[3], r[4]]);
        y_data.push(r[5]);
    }

    let (x_train, x_test) = x_data.split_at(30);
    let (y_train, y_test) = y_data.split_at(30);

    let initial = [0., 1., 1., 1., 1., 1.];
    let lambdas = [0., 1., 2., 3., 4., 5.];
    let mut train_mses = vec![];
    let mut test_mses = vec![];

    for l in lambdas {
        let params = optimize(
            initial,
            1000,
            0.001,
            [0., l, l, l, l, l],
            new_linear_gradient(x_train, y_train),
            l1,
            |_, _| {},
        );
        let c = params[0];
        let ms = params[1..].try_into().unwrap();
        train_mses.push((l, mse(|x| multi_linear(c, ms, x), x_train, y_train)));
        test_mses.push((l, mse(|x| multi_linear(c, ms, x), x_test, y_test)));
    }

    write_csv(&train_mses, TRAIN_OUTPUT_PATH)?;
    write_csv(&test_mses, TEST_OUTPUT_PATH)?;

    Ok(())
}

fn write_csv<T: Serialize>(data: &[T], path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(path)?;
    for r in data {
        writer.serialize(r)?;
    }
    Ok(())
}
