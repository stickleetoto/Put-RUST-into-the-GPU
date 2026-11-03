//! CPU machine learning baseline; Burn integration is planned.
use ferrum_core::ComputeError;

pub fn mean_squared_error(predictions: &[f32], targets: &[f32]) -> Result<f32, ComputeError> {
    if predictions.len() != targets.len() {
        return Err(ComputeError::LengthMismatch {
            left: predictions.len(),
            right: targets.len(),
        });
    }
    if predictions.is_empty() {
        return Err(ComputeError::EmptyInput);
    }
    Ok(predictions
        .iter()
        .zip(targets)
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        / predictions.len() as f32)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LinearRegression {
    pub weight: f32,
    pub bias: f32,
}

impl LinearRegression {
    pub fn predict(&self, x: f32) -> f32 {
        self.weight * x + self.bias
    }

    pub fn train(
        &mut self,
        samples: &[(f32, f32)],
        learning_rate: f32,
        epochs: usize,
    ) -> Result<(), ComputeError> {
        if samples.is_empty() {
            return Err(ComputeError::EmptyInput);
        }
        if !learning_rate.is_finite() || learning_rate <= 0.0 {
            return Err(ComputeError::InvalidParameter("learning_rate"));
        }
        let scale = 2.0 / samples.len() as f32;
        for _ in 0..epochs {
            let mut weight_gradient = 0.0;
            let mut bias_gradient = 0.0;
            for &(x, y) in samples {
                let error = self.predict(x) - y;
                weight_gradient += error * x;
                bias_gradient += error;
            }
            self.weight -= learning_rate * scale * weight_gradient;
            self.bias -= learning_rate * scale * bias_gradient;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn learns_linear_mapping() {
        let mut model = LinearRegression::default();
        let samples = [(0.0, 1.0), (1.0, 3.0), (2.0, 5.0), (3.0, 7.0)];
        model.train(&samples, 0.05, 1000).unwrap();
        assert!((model.predict(4.0) - 9.0).abs() < 0.1);
        assert_eq!(mean_squared_error(&[1.0, 3.0], &[0.0, 1.0]).unwrap(), 2.5);
    }
}
