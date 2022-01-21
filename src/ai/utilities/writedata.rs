use csv::{Error, Writer};
use serde::Serialize;

pub fn write_data<S, I>(path: &str, iterable: I) -> Result<(), Error>
where
    S: Serialize,
    I: IntoIterator<Item = S>,
{
    let mut writer = Writer::from_path(path)?;
    for record in iterable {
        writer.serialize(record)?;
    }
    Ok(())
}
