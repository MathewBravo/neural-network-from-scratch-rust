use crate::math::{dotprod::DotProduct, errors::MathError};

#[derive(Debug, PartialEq)]
pub struct Neuron {
    pub inputs: Vec<f32>,
    pub weights: Vec<f32>,
    pub bias: f32,
}

impl Neuron {
    pub fn weighted_sum(&self) -> Result<f32, MathError> {
        let dp: Vec<f32> = self.weights.dot_prod(&self.inputs)?;
        let sum = dp[0] + self.bias;
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::Neuron;

    #[test]
    fn init_neuron() {
        let test_neuron = Neuron {
            inputs: vec![1.0, 2.0, 3.0],
            weights: vec![0.2, 0.8, -0.5],
            bias: 2.0,
        };
        assert_eq!(2.0, test_neuron.inputs[1]);
        assert_eq!(0.2, test_neuron.weights[0]);
        assert_eq!(2.0, test_neuron.bias);
    }

    #[test]
    fn neuron_weighted_sum() {
        let test_neuron = Neuron {
            inputs: vec![1.0, 2.0, 3.0, 2.5],
            weights: vec![0.2, 0.8, -0.5, 1.0],
            bias: 4.8,
        };
        assert_eq!(7.6000004, test_neuron.weighted_sum().unwrap());
    }
}
