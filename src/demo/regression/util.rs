use csv::Writer;
use serde::Serialize;
use std::error::Error;

pub fn write_csv<T: Serialize>(data: &[T], path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(path)?;
    for r in data {
        writer.serialize(r)?;
    }
    Ok(())
}
