//! Host-side timing only. CUDA event timing is planned for real GPU kernels.
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkReport {
    pub iterations: usize,
    pub mean_ns: u128,
    pub p50_ns: u128,
    pub p95_ns: u128,
    pub p99_ns: u128,
}

fn percentile(sorted: &[u128], fraction: f64) -> u128 {
    if sorted.is_empty() { return 0; }
    let rank = (fraction * sorted.len() as f64).ceil() as usize;
    sorted[rank.saturating_sub(1).min(sorted.len() - 1)]
}

impl BenchmarkReport {
    pub fn from_samples(samples: &[u128]) -> Self {
        if samples.is_empty() {
            return Self { iterations: 0, mean_ns: 0, p50_ns: 0, p95_ns: 0, p99_ns: 0 };
        }
        let mut ordered = samples.to_vec();
        ordered.sort_unstable();
        Self {
            iterations: ordered.len(),
            mean_ns: ordered.iter().sum::<u128>() / ordered.len() as u128,
            p50_ns: percentile(&ordered, 0.50),
            p95_ns: percentile(&ordered, 0.95),
            p99_ns: percentile(&ordered, 0.99),
        }
    }
}

pub fn measure<F: FnMut()>(iterations: usize, mut operation: F) -> BenchmarkReport {
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        operation();
        samples.push(start.elapsed().as_nanos());
    }
    BenchmarkReport::from_samples(&samples)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn percentiles_and_empty_sample() {
        let result = BenchmarkReport::from_samples(&[5, 1, 3, 2, 4]);
        assert_eq!(result.mean_ns, 3);
        assert_eq!(result.p50_ns, 3);
        assert_eq!(result.p95_ns, 5);
        assert_eq!(measure(0, || {}).iterations, 0);
    }
}
