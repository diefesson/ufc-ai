mod kmeansdemo;
mod knndemo;

pub use kmeansdemo::*;
pub use knndemo::*;

const DATA_PATH: &str = "data/iris.data";

type DataType = (f64, f64, f64, f64, String);
