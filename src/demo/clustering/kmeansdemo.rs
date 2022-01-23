use super::{DataType, DATA_PATH};
use crate::ai::clustering::k_means;
use crate::ai::utilities::load_with_transform;
use std::error::Error;

pub fn k_means_demo() -> Result<(), Box<dyn Error>> {
    println!("loading data: {}...", DATA_PATH);
    let samples = load_with_transform::<DataType, _, _>(DATA_PATH, |d| [d.0, d.1, d.2, d.3])?;
    println!("loaded {} samples", samples.len());
    let mut centers: Vec<[f64; 4]> = samples[0..3].into();
    println!("start centers: {:?}", centers);
    println!("finding new centers...");
    centers = k_means(centers, &samples, 100);
    println!("final centers: {:?}", centers);
    Ok(())
}
