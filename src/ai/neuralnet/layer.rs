use super::NDArray;

pub trait Layer {
    fn get_input_shape(&self) -> &[usize];
    fn get_output_shape(&self) -> &[usize];
    fn update(&self, input: &NDArray<f64>, output: &mut NDArray<f64>);
}

pub trait WeightedLayer: Layer {
    fn get_weights_shapes(&self) -> &[usize];
    fn get_weights(&self) -> &NDArray<f64>;
    fn set_weights(&mut self, weights: NDArray<f64>);
}
