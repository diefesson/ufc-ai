use crate::leastsquares::least_squares;
use csv::Reader;
use std::error::Error;

const DATASET_PATH: &str = "data/ex1data2.txt";

pub fn demo_2_ls() -> Result<(), Box<dyn Error>> {
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    for r in Reader::from_path(DATASET_PATH)?.deserialize() {
        let r: (f64, f64, f64) = r?;
        x_data.push([r.0, r.1]);
        y_data.push(r.2);
    }

    let coeficients: [f64; 3] = least_squares(&x_data, &y_data, 0.);

    println!("{:?}", coeficients);

    Ok(())
}
