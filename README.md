# QCGPU

> An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust

* Uses OpenCL kernels.
* Runs Cross Platform.

## Prerequisites:
* OpenCL (Ensure that an OpenCL library is installed for your platform and that `clinfo` or some other diagnostic command will run). 
* Rust (install [here](https://www.rustup.rs)). Please use a nightly build.

## Initial Benchmarks

```rust
test tests::qubits_20_not_all_cpu ... bench: 166,565,996 ns/iter (+/- 4,235,438)
test tests::qubits_20_not_all_gpu ... bench:  74,625,368 ns/iter (+/- 4,295,963)
```

## Notes

A stretch goal for this project is to also (optionally) simulate decoherence, to allow
for more realistic simulations.

## License

This software is licensed under the MIT licence (see `LICENSE.md`)

Copyright (c) 2018 Adam Kelly


