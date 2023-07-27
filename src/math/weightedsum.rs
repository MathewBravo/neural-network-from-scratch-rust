use crate::math::errors::MathError;

fn weighted_sum(dot_product: Vec<f32>, biases: Vec<f32>) -> Result<Vec<f32>, MathError> {
    if dot_product.len() != biases.len() {
        Err(MathError::NotVectorSameLength)
    } else {
        let w_sum: Vec<f32> = dot_product
            .iter()
            .zip(biases.iter())
            .map(|(&x, &y)| x + y)
            .collect();
        Ok(w_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_sum_same_length() {
        let dot_product = vec![1.0, 2.5, 3.0, 4.5];
        let biases = vec![0.5, 1.0, 1.5, 2.0];
        let expected_result = vec![1.5, 3.5, 4.5, 6.5];

        let result = weighted_sum(dot_product, biases);

        assert_eq!(result, Ok(expected_result));
    }

    #[test]
    fn test_weighted_sum_different_length() {
        let dot_product = vec![1.0, 2.5, 3.0];
        let biases = vec![0.5, 1.0, 1.5, 2.0];

        let result = weighted_sum(dot_product, biases);

        assert_eq!(result, Err(MathError::NotVectorSameLength));
    }
}
