use ferrum_bench::measure;
use ferrum_cpu::vector_add;
use ferrum_cuda::probe_environment;
use ferrum_experiment::ExperimentRecord;
use ferrum_ml::LinearRegression;

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("demo") => run_demo(),
        Some("probe") => run_probe(),
        _ => {
            println!("Put RUST into the GPU - development scaffold");
            println!("Usage: cargo run -p ferrum-cli -- demo");
            println!("       cargo run -p ferrum-cli -- probe");
        }
    }
}

fn run_demo() {
    let output = vector_add(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).unwrap();
    println!("CPU vector addition: {output:?}");

    let bench = measure(100, || {
        let output = vector_add(&[1.0, 2.0], &[3.0, 4.0]).unwrap();
        std::hint::black_box(output);
    });
    println!("Host timing (includes allocation): {bench:?}");

    let mut model = LinearRegression::default();
    model.train(&[(0.0, 1.0), (1.0, 3.0), (2.0, 5.0), (3.0, 7.0)], 0.05, 1000).unwrap();
    let mut experiment = ExperimentRecord::new("cpu-linear-regression");
    experiment.add_metric("prediction_at_4", f64::from(model.predict(4.0)));
    experiment.add_metric("host_p95_ns", bench.p95_ns as f64);
    println!("Experiment record: {}", experiment.to_json());
}

fn run_probe() {
    let env = probe_environment();
    println!("CUDA toolkit (nvcc) available: {}", env.nvcc_available);
    println!("NVIDIA management utility available: {}", env.nvidia_smi_available);
    println!("This does not test GPU kernel execution.");
}
