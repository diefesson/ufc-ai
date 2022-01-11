use crate::ai::neuralnet::activations::relu;
use crate::ai::neuralnet::layers::Dense;
use crate::neuralnet::{Layer, NDArray, WeightedLayer};
use std::error::Error;

pub fn playground() -> Result<(), Box<dyn Error>> {
    let weights = NDArray::with_shape_and_data(vec![1, 2], vec![0.5, 0.5]);
    let input = vec![0.5, 0.5].into();
    let mut output = NDArray::with_shape(&[1]);

    let mut layer = Dense::new(2, 1, relu);
    layer.set_weights(weights);
    layer.update(&input, &mut output);

    println!("{:?}", output.get_data());
    Ok(())
}
