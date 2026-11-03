# Roadmap

## Milestone 0: Foundation
- [x] Cargo workspace and initial crates
- [x] CPU vector operations and tests
- [x] CUDA tool availability probe (not kernel execution)
- [x] Host timing percentiles
- [x] PyO3 binding with Python list inputs
- [x] Linear regression and JSON experiment records
- [x] CI workflow configuration
- [ ] Confirm CI passes on GitHub
- [ ] Run on a Windows NVIDIA machine

## Milestone 1: GPU DevKit (A)
- [ ] CUDA C++ vector-add kernel behind an opt-in feature
- [ ] Device buffers and transfers
- [ ] CUDA event timing and CPU-reference correctness tests
- [ ] RTX 4050 / RTX 3060 benchmarks

## Milestone 2: Python Accelerator (C)
- [ ] NumPy array interfaces and copy analysis
- [ ] DLPack feasibility and PyTorch bridge
- [ ] Windows Python wheel tests

## Milestone 3: ML Toolkit (B)
- [ ] Burn backend adapter evaluation
- [ ] Dataset loading and experiment reproducibility
- [ ] MLP training and GPU profiling

## Milestone 4: Public alpha
- [ ] Benchmarked documented samples
- [ ] Stable public API and release checklist
