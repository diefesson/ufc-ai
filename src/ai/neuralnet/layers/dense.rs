use crate::neuralnet::activations::Activation;
use crate::neuralnet::{Layer, NDArray, ShapeSize, WeightedLayer};

pub struct Dense {
    input_shape: Vec<usize>,
    output_shape: Vec<usize>,
    activation: Activation,
    weights: NDArray<f64>,
}

impl Dense {
    pub fn new(input_size: usize, ouput_size: usize, activation: Activation) -> Self {
        Self {
            input_shape: vec![input_size],
            output_shape: vec![ouput_size],
            activation: activation,
            weights: NDArray::with_shape(&[ouput_size, input_size]),
        }
    }
}

impl Layer for Dense {
    fn get_input_shape(&self) -> &[usize] {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &[usize] {
        &self.output_shape
    }

    fn update(&self, input: &NDArray<f64>, output: &mut NDArray<f64>) {
        assert_eq!(self.input_shape, input.get_shape(), "input shape mismatch");
        assert_eq!(
            self.output_shape,
            output.get_shape(),
            "output shape mismatch"
        );

        for (oi, o) in output.iter_mut().enumerate() {
            let x = input
                .iter()
                .enumerate()
                .map(|(ii, i)| self.weights[[oi, ii]] * i)
                .sum::<f64>()
                / self.input_shape.size() as f64;
            *o = (self.activation)(x);
        }
    }
}

impl WeightedLayer for Dense {
    fn get_weights_shapes(&self) -> &[usize] {
        self.weights.get_shape()
    }

    fn get_weights(&self) -> &NDArray<f64> {
        &self.weights
    }

    fn set_weights(&mut self, new_weights: NDArray<f64>) {
        assert_eq!(
            self.weights.get_shape(),
            new_weights.get_shape(),
            "weight shape mismatch"
        );
        self.weights = new_weights;
    }
}
