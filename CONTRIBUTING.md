# Contributing

Stable Rust, rustfmt and clippy are required. A GPU is not required for current tests.

~~~bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p ferrum-cli -- demo
~~~

To work on the optional Python bindings, install Python 3.9+ and maturin, then run `maturin build` at the repository root.

New numerical kernels must include correctness tests and reproducible performance measurements. Keep GPU-specific work optional for CPU-only contributors.
