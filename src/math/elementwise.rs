use crate::math::errors::MathError;

pub fn element_wise_sum(vec1: &[f32], vec2: &[f32]) -> Result<f32, MathError> {
    if vec1.len() != vec2.len() {
        return Err(MathError::NotVectorSameLength);
    }
    let wise_sum = vec1
        .iter()
        .zip(vec2.iter())
        .fold(0.0, |acc, (&x, &y)| acc + x + y);
    Ok(wise_sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_element_wise_sum_success() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];

        let result = element_wise_sum(&vec1, &vec2);

        assert_eq!(result, Ok(21.0));
    }

    #[test]
    fn test_element_wise_sum_failure_different_lengths() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0]; // Different length vectors

        let result = element_wise_sum(&vec1, &vec2);

        assert_eq!(result, Err(MathError::NotVectorSameLength));
    }
}
