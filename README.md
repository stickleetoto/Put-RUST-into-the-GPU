# Put RUST into the GPU

**A Rust-first toolkit for GPU computing, Python acceleration, and machine learning.**

> **Status: initial scaffold.** CPU reference kernels and NVIDIA tool diagnostics work in principle; actual CUDA GPU kernels are **not yet implemented**.

| Track | Goal | Initial functionality |
| --- | --- | --- |
| **A · GPU DevKit** | GPU kernels, profiling, benchmarking | CPU reference operations, host timing, CUDA environment probe |
| **C · Python ↔ Rust** | Python-callable acceleration | PyO3 Python-list vector addition |
| **B · ML Toolkit** | Model training experiments | CPU linear regression, JSON metric records |

## Quick start

Install stable Rust and run:

~~~bash
git clone https://github.com/stickleetoto/Put-RUST-into-the-GPU.git
cd Put-RUST-into-the-GPU
cargo test --workspace
cargo run -p ferrum-cli -- demo
cargo run -p ferrum-cli -- probe
~~~

The CLI demo runs simple CPU operations and a tiny linear regression experiment. `probe` only checks whether NVIDIA command-line utilities can execute.

## Python preview

Install Python 3.9+ and maturin:

~~~bash
python -m pip install maturin
maturin build --release
~~~

Install the generated wheel from `target/wheels/`, then:

~~~python
import ferrum_compute as fc
print(fc.vector_add([1.0, 2.0], [3.0, 4.0]))
# [4.0, 6.0]
~~~

This currently converts Python lists to Rust vectors; zero-copy NumPy integration is future work.

## Modules

- `ferrum-core`: shared device types and errors
- `ferrum-cpu`: reference CPU kernels
- `ferrum-cuda`: NVIDIA tool diagnostics, not GPU execution
- `ferrum-bench`: host-side latency statistics
- `ferrum-python`: optional PyO3 binding
- `ferrum-ml`: CPU training baseline
- `ferrum-experiment`: JSON metrics
- `ferrum-cli`: runnable examples

Read [architecture](docs/ARCHITECTURE.md), [roadmap](docs/ROADMAP.md), and [contributing](CONTRIBUTING.md).

**Platform plan:** Windows-first NVIDIA GPU development with Linux/Windows CPU-only CI. CUDA C++ kernels first, native Rust GPU kernel approaches later. Performance claims must come from measured reproducible GPU workloads.

**License:** MIT. See [LICENSE](LICENSE).
