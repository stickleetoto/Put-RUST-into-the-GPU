//! Reference CPU kernels for correctness comparisons.
use ferrum_core::ComputeError;

pub fn vector_add(left: &[f32], right: &[f32]) -> Result<Vec<f32>, ComputeError> {
    if left.len() != right.len() {
        return Err(ComputeError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }
    Ok(left.iter().zip(right).map(|(a, b)| a + b).collect())
}

pub fn dot(left: &[f32], right: &[f32]) -> Result<f32, ComputeError> {
    if left.len() != right.len() {
        return Err(ComputeError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }
    Ok(left.iter().zip(right).map(|(a, b)| a * b).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_and_dot() {
        assert_eq!(
            vector_add(&[1.0, 2.0], &[3.0, 4.0]).unwrap(),
            vec![4.0, 6.0]
        );
        assert_eq!(dot(&[1.0, 2.0], &[3.0, 4.0]).unwrap(), 11.0);
        assert!(vector_add(&[1.0], &[2.0, 3.0]).is_err());
    }
}
