use std::error::Error;

use crate::clustering::k_means;

use super::util::load_data;

const DATA_PATH: &str = "data/iris.data";

type DataType = (f64, f64, f64, f64, String);
type SampleType = [f64; 4];

pub fn demo() -> Result<(), Box<dyn Error>> {
    println!("loading data: {}...", DATA_PATH);
    let samples = load_data::<DataType>(DATA_PATH)?
        .iter()
        .map(|d| [d.0, d.1, d.2, d.3])
        .collect::<Vec<SampleType>>();
    println!("loaded {} samples", samples.len());
    let mut centers: Vec<SampleType> = samples[0..3].into();
    println!("start centers: {:?}", centers);
    println!("finding new centers...");
    centers = k_means(centers, &samples, 100);
    println!("final centers: {:?}", centers);
    Ok(())
}
