use crate::functions::{mse, multi_linear};
use crate::gradient::regularizations::no_regularization;
use crate::gradient::{gradients::new_linear_gradient, optimize};
use csv::{Reader, Writer};
use std::convert::TryInto;
use std::error::Error;

const DATASET_PATH: &str = "data/ex1data2.txt";
const OUTPUT_PATH: &str = "output/output-2.csv";

pub fn demo_2() -> Result<(), Box<dyn Error>> {
    let mut x_data = vec![];
    let mut y_data = vec![];
    for r in Reader::from_path(DATASET_PATH)?.deserialize() {
        let r: [f64; 3] = r?;
        x_data.push([r[0], r[1]]);
        y_data.push(r[2]);
    }

    let mut mses = Vec::<f64>::new();

    let params = optimize(
        [0., 0., 0.],
        100,
        0.01,
        Default::default(),
        new_linear_gradient(&x_data, &y_data),
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
