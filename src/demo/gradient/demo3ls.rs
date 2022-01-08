use super::util::write_csv;
use crate::functions::{mse, multi_linear};
use crate::leastsquares::least_squares;
use csv::Reader;
use std::{convert::TryInto, error::Error};

const DATASET_PATH: &str = "data/ex1data3.txt";
const TRAIN_OUTPUT_PATH: &str = "output/output-3-ls-train.csv";
const TEST_OUTPUT_PATH: &str = "output/output-3-ls-test.csv";

pub fn demo_3_ls() -> Result<(), Box<dyn Error>> {
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    for r in Reader::from_path(DATASET_PATH)?.deserialize() {
        let r: [f64; 6] = r?;
        x_data.push([r[0], r[1], r[2], r[3], r[4]]);
        y_data.push(r[5]);
    }

    let (x_train, x_test) = x_data.split_at(30);
    let (y_train, y_test) = y_data.split_at(30);
    let lambdas = [0., 1., 2., 3., 4., 5.];
    let mut train_mses = vec![];
    let mut test_mses = vec![];

    for l in lambdas {
        let params: [f64; 6] = least_squares(&x_train, &y_train, l);
        let c = params[0];
        let ms = params[1..].try_into().unwrap();
        train_mses.push((l, mse(|x| multi_linear(c, ms, x), x_train, y_train)));
        test_mses.push((l, mse(|x| multi_linear(c, ms, x), x_test, y_test)));
    }

    write_csv(&train_mses, TRAIN_OUTPUT_PATH)?;
    write_csv(&test_mses, TEST_OUTPUT_PATH)?;

    Ok(())
}
