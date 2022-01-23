use super::{DataType, DATA_PATH};
use crate::clustering::knn;
use crate::functions::classification_accuracy;
use crate::utilities::{encode_labels, load_interactively, SplitAtRatio};
use std::error::Error;

pub fn knn_demo() -> Result<(), Box<dyn Error>> {
    println!("loading data...");
    let mut x_data = vec![];
    let mut y_data = vec![];
    load_interactively::<DataType, _>(DATA_PATH, |d| {
        x_data.push([d.0, d.1, d.2, d.3]);
        y_data.push(d.4)
    })?;
    println!("loaded {} samples", x_data.len());
    let (y_labels, mapping) = encode_labels(&y_data);
    println!("found {} labels", mapping.len());
    let (x_train, x_test) = x_data.split_at_ratio(0.8);
    let (y_train, y_test) = y_labels.split_at_ratio(0.8);
    println!(
        "using {} train samples and {} test samples",
        x_train.len(),
        x_data.len() - x_train.len()
    );
    println!("calculating accuracy...");
    let accuracy = classification_accuracy(|x| knn(x_train, y_train, 10, x), x_test, y_test);
    println!("accuracy: {}", accuracy);
    Ok(())
}
