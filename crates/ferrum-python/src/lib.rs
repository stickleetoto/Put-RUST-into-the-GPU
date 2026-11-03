//! Optional Python extension through PyO3.
pub fn vector_add_cpu(left: &[f32], right: &[f32]) -> Result<Vec<f32>, String> {
    ferrum_cpu::vector_add(left, right).map_err(|error| error.to_string())
}

#[cfg(feature = "python")]
mod python {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    #[pyfunction]
    fn vector_add(left: Vec<f32>, right: Vec<f32>) -> PyResult<Vec<f32>> {
        super::vector_add_cpu(&left, &right).map_err(PyValueError::new_err)
    }

    #[pymodule]
    fn ferrum_compute(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add_function(wrap_pyfunction!(vector_add, module)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn computes_through_cpu_bridge() {
        assert_eq!(super::vector_add_cpu(&[1.0], &[2.0]).unwrap(), vec![3.0]);
    }
}
