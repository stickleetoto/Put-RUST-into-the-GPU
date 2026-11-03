# Architecture

**Put RUST into the GPU** is an early-stage Rust-first GPU and ML development toolkit.

- **Shared**: `ferrum-core` contains error types and device identifiers.
- **A / GPU DevKit**: `ferrum-cpu` supplies reference kernels, `ferrum-cuda` probes NVIDIA tools, and `ferrum-bench` provides host-side latency summaries.
- **C / Accelerator**: `ferrum-python` exposes a limited Rust API via optional PyO3 bindings.
- **B / ML Toolkit**: `ferrum-ml` contains a reference CPU learning algorithm and `ferrum-experiment` serializes results.
- **Integration**: `ferrum-cli` demonstrates the existing features.

The core has no dependency on Python or CUDA. CUDA C++ kernel execution, CUDA events, NumPy/DLPack interoperability, and Burn-backed training are future work.

## Benchmark policy

Do not equate host-side elapsed time with GPU kernel time. Future GPU measurements should separate allocation, host/device transfer, kernel execution, synchronization, and end-to-end latency. Include input shapes, hardware/driver, build profile, warmup, sampling counts, and numerical reference tolerances.
