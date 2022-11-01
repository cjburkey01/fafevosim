//! Generic neural network that be used with any kind of float.
//!
//! (Most) neural network code has been "borrowed" from: https://github.com/jackm321/RustNN
//! under the Apache license, available from the repository here:
//! https://github.com/jackm321/RustNN/blob/c159117494b813f3558f428037d0c45b949b89cf/LICENSE-APACHE
//! or directly from [apache.org](https://www.apache.org/licenses/LICENSE-2.0.txt).

use num_traits::Float as NumFloat;
use rand::{distributions::uniform::SampleUniform, Rng};
use std::ops::{AddAssign, Deref, DerefMut};

/// A generic activation function (so they may be implemented elsewhere).
pub trait ActivationFunction<Float: NumFloat> {
    /// Performs the activation function on the given value.
    fn perform(&self, val: Float) -> Float;
}

/// Default activation function(s).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NNActivation {
    /// The sigmoid activation function.
    Sigmoid,
}

impl<Float: NumFloat> ActivationFunction<Float> for NNActivation {
    /// Perform the activation function.
    fn perform(&self, val: Float) -> Float {
        match self {
            Self::Sigmoid => Float::one() / (Float::one() + (-val).exp()),
        }
    }
}

/// A single node in a neural network (its weights and bias).
///
/// Dereferences to the internal vector of weights.
#[derive(Debug, Clone)]
pub struct NNNode<Float: NumFloat>(pub Vec<Float>);

impl<Float: NumFloat> Deref for NNNode<Float> {
    type Target = Vec<Float>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Float: NumFloat> DerefMut for NNNode<Float> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A single layer in a neural network (its nodes).
///
/// Dereferences to the internal vector of nodes.
#[derive(Debug, Clone)]
pub struct NNLayer<Float: NumFloat>(pub Vec<NNNode<Float>>);

impl<Float: NumFloat> Deref for NNLayer<Float> {
    type Target = Vec<NNNode<Float>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Float: NumFloat> DerefMut for NNLayer<Float> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NNCreateError {
    #[error("neural network must have at least an input layer and output layer")]
    Min2Layers,

    #[error("each layer of neural network must have at least one node")]
    EmptyLayer,
}

/// A neural network.
#[derive(Debug, Clone)]
pub struct NN<Float: NumFloat> {
    layers: Vec<NNLayer<Float>>,
    num_inputs: u32,
}

impl<Float: NumFloat + SampleUniform + AddAssign> NN<Float> {
    /// Create a new neural network with the given layer sizes. The first
    /// layer size provided will be the number of inputs, the last will be
    /// the number of outputs.
    /// There must be at least two elements in this `layer_sizes` slice.
    pub fn random(layers_sizes: &[u32]) -> Result<NN<Float>, NNCreateError> {
        // Make sure there is at least an input layer and an output layer
        if layers_sizes.len() < 2 {
            return Err(NNCreateError::Min2Layers);
        }

        // Make sure all layers have at least one node
        for &layer_size in layers_sizes.iter() {
            if layer_size < 1 {
                return Err(NNCreateError::EmptyLayer);
            }
        }

        let mut layers = Vec::new();
        let mut it = layers_sizes.iter();
        // get the first layer size
        let first_layer_size = *it.next().unwrap();

        let mut rng = rand::thread_rng();

        // setup the rest of the layers
        let mut prev_layer_size = first_layer_size;
        for &layer_size in it {
            let mut layer = NNLayer(Vec::new());
            for _ in 0..layer_size {
                let mut node = NNNode(Vec::new());
                for _ in 0..prev_layer_size + 1 {
                    let random_weight =
                        rng.gen_range(Float::from(-0.5).unwrap()..=Float::from(0.5).unwrap());
                    node.push(random_weight);
                }
                node.shrink_to_fit();
                layer.push(node)
            }
            layer.shrink_to_fit();
            layers.push(layer);
            prev_layer_size = layer_size;
        }
        layers.shrink_to_fit();
        Ok(NN {
            layers,
            num_inputs: first_layer_size,
        })
    }

    /// Runs the neural network and returns the output layer values.
    /// Returns `Result::Err` when the input size does not match the number of
    /// inputs for this neural network.
    pub fn run<Activation: ActivationFunction<Float>>(
        &self,
        activation: Activation,
        inputs: &[Float],
    ) -> Result<Vec<Float>, ()> {
        Ok(self.do_run(activation, inputs)?.pop().unwrap())
    }

    /// Runs the neural network and returns all layer results.
    /// Returns `Result::Err` when the input size does not match the number of
    /// inputs for this neural network.
    fn do_run<Activation: ActivationFunction<Float>>(
        &self,
        activation: Activation,
        inputs: &[Float],
    ) -> Result<Vec<Vec<Float>>, ()> {
        // Function to calculate a single node's value from the previous layer
        // values
        fn modified_dotprod<Float: NumFloat + AddAssign>(
            node: &[Float],
            values: &[Float],
        ) -> Float {
            let mut it = node.iter();
            let mut total = *it.next().unwrap(); // start with the threshold weight
            for (weight, value) in it.zip(values.iter()) {
                total += *weight * *value;
            }
            total
        }

        // Size check
        if inputs.len() as u32 == self.num_inputs {
            // Create results and push inputs to begin processing
            let mut results = Vec::with_capacity(self.layers.len());
            results.push(inputs.to_vec());

            // Loop through each layer and add it to the results vector.
            for (layer_index, layer) in self.layers.iter().enumerate() {
                let mut layer_results = Vec::new();
                for node in layer.iter() {
                    layer_results
                        .push(activation.perform(modified_dotprod(node, &results[layer_index])))
                }
                results.push(layer_results);
            }
            Ok(results)
        } else {
            Err(())
        }
    }
}
