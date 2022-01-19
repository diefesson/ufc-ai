use csv::Reader;
use serde::de::DeserializeOwned;
use std::error::Error;

pub fn load_data<T: DeserializeOwned>(path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut data = vec![];
    for r in reader.deserialize() {
        data.push(r?);
    }
    Ok(data)
}
