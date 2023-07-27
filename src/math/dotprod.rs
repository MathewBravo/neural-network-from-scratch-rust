use crate::math::errors::MathError;

pub trait DotProduct {
    fn dot_prod(&self, inputs: &[f32]) -> Result<Vec<f32>, MathError>;
}

impl DotProduct for Vec<f32> {
    fn dot_prod(&self, inputs: &[f32]) -> Result<Vec<f32>, MathError> {
        if self.len() != inputs.len() {
            return Err(MathError::NotVectorSameLength);
        }
        let dot_product = self
            .iter()
            .zip(inputs.iter())
            .fold(0.0, |acc, (&x, &y)| acc + x * y);
        Ok(vec![dot_product])
    }
}

impl DotProduct for Vec<Vec<f32>> {
    fn dot_prod(&self, inputs: &[f32]) -> Result<Vec<f32>, MathError> {
        if self.is_empty() || self[0].len() != inputs.len() {
            return Err(MathError::NotVectorSameLength);
        }

        let mut dot_product = vec![0.0; self.len()];
        for (i, weights_row) in self.iter().enumerate() {
            if weights_row.len() != inputs.len() {
                return Err(MathError::NotVectorSameLength);
            }

            dot_product[i] = weights_row
                .iter()
                .zip(inputs.iter())
                .fold(0.0, |acc, (&x, &y)| acc + x * y);
        }

        Ok(dot_product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_prod_success_1d() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];

        let result = vec1.dot_prod(&vec2).unwrap();

        assert_eq!(result, vec![32.0]);
    }

    #[test]
    fn test_dot_prod_success_2d() {
        let weights = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let inputs = vec![4.0, 5.0, 6.0];

        let result = weights.dot_prod(&inputs).unwrap();

        assert_eq!(result, vec![32.0, 77.0, 122.0]);
    }

    #[test]
    fn test_dot_prod_failure_1d() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0]; // Different length vectors

        let result = vec1.dot_prod(&vec2);

        assert_eq!(result, Err(MathError::NotVectorSameLength));
    }

    #[test]
    fn test_dot_prod_failure_2d() {
        let weights = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0], vec![7.0, 8.0, 9.0]]; // Different length vectors
        let inputs = vec![4.0, 5.0, 6.0];

        let result = weights.dot_prod(&inputs);

        assert_eq!(result, Err(MathError::NotVectorSameLength));
    }
}
