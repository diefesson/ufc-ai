use csv::{Error, Reader};
use serde::de::DeserializeOwned;
use std::convert::identity;

pub fn load_interactively<D, O>(path: &str, mut on_data: O) -> Result<(), Error>
where
    D: DeserializeOwned,
    O: FnMut(D),
{
    let mut reader = Reader::from_path(path)?;
    for d in reader.deserialize() {
        on_data(d?);
    }
    Ok(())
}

pub fn load_with_transform<I, O, T>(path: &str, mut transform: T) -> Result<Vec<O>, Error>
where
    I: DeserializeOwned,
    T: FnMut(I) -> O,
{
    let mut data = vec![];
    load_interactively(path, |d| data.push(transform(d)))?;
    Ok(data)
}

pub fn load_data<D: DeserializeOwned>(path: &str) -> Result<Vec<D>, Error> {
    load_with_transform(path, identity)
}
