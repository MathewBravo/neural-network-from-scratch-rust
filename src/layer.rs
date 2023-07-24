use crate::neuron::Neuron;

#[derive(Debug, PartialEq)]
pub enum LayerOperationalErrors {
    WeightBiasError,
}

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub size: u32,
    pub neurons: Vec<Neuron>,
}

impl Layer {
    fn weighted_sums(self) -> Vec<f32> {
        let mut sums: Vec<f32> = Vec::with_capacity(self.neurons.len());
        for neuron in self.neurons {
            sums.push(neuron.weighted_sum());
        }
        sums
    }
}

pub fn create_layer(
    inputs: Vec<f32>,
    weights: Vec<Vec<f32>>,
    biases: Vec<f32>,
) -> Result<Layer, LayerOperationalErrors> {
    if weights.len() != biases.len() {
        println!(
            "Error creating neurons, {} weights given with {} biases",
            weights.len(),
            biases.len()
        );
        return Err(LayerOperationalErrors::WeightBiasError);
    } else {
        let mut new_layer = Layer {
            size: weights.len() as u32,
            neurons: vec![],
        };
        let range = weights.len();
        for i in 0..range {
            let new_neuron = Neuron {
                inputs: inputs.clone(),
                weights: weights[i].clone(),
                bias: biases[i],
            };
            new_layer.neurons.push(new_neuron);
        }
        Ok(new_layer)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        layer::{create_layer, Layer},
        neuron::Neuron,
    };

    #[test]
    fn test_weighted_sums() {
        let test_nuer1 = Neuron {
            inputs: vec![1.0, 2.0, 3.0, 2.5],
            weights: vec![0.2, 0.8, -0.5, 1.0],
            bias: 2.0,
        };
        let test_nuer2 = Neuron {
            inputs: vec![1.0, 2.0, 3.0, 2.5],
            weights: vec![0.5, -0.91, 0.26, -0.5],
            bias: 3.0,
        };
        let test_nuer3 = Neuron {
            inputs: vec![1.0, 2.0, 3.0, 2.5],
            weights: vec![-0.26, -0.27, 0.17, 0.87],
            bias: 0.5,
        };
        let test_layer = Layer {
            size: 3,
            neurons: vec![test_nuer1, test_nuer2, test_nuer3],
        };
        let test_vec = vec![4.8, 1.2099999, 2.385];
        assert_eq!(test_vec, test_layer.weighted_sums())
    }
    #[test]
    fn test_create_neurons() {
        let inputs = vec![1.0, 2.0, 3.0, 2.5];
        let weights = vec![
            vec![0.2, 0.8, -0.5, 1.0],
            vec![0.5, -0.91, 0.26, -0.5],
            vec![-0.26, -0.27, 0.17, 0.87],
        ];
        let biases = vec![2.0, 3.0, 0.5];

        let result = create_layer(inputs.clone(), weights.clone(), biases.clone());

        let expected_layer = Layer {
            size: 3,
            neurons: vec![
                Neuron {
                    inputs: inputs.clone(),
                    weights: weights[0].clone(),
                    bias: biases[0],
                },
                Neuron {
                    inputs: inputs.clone(),
                    weights: weights[1].clone(),
                    bias: biases[1],
                },
                Neuron {
                    inputs: inputs.clone(),
                    weights: weights[2].clone(),
                    bias: biases[2],
                },
            ],
        };

        assert_eq!(result, Ok(expected_layer));
    }
}
