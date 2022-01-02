use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

pub fn csv_test() -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_reader(File::open("data/ex1data1.txt")?);
    reader.set_headers(vec!["x", "y"].into());
    for result in reader.deserialize() {
        let record: Point = result?;
        println!("{:?}", record);
    }
    Ok(())
}
