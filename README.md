# QCGPU

> An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust

* Uses OpenCL kernels.
* Runs Cross Platform.

## Prerequisites:
* OpenCL (Ensure that an OpenCL library is installed for your platform and that `clinfo` or some other diagnostic command will run). 
* Rust (install [here](https://www.rustup.rs)). Please use a nightly build.

## Initial Benchmarks

```rust
test tests::qubits_18_not_all_cpu ... bench:  41,712,028 ns/iter (+/- 12,006,823)
test tests::qubits_18_not_all_gpu ... bench:  20,557,968 ns/iter (+/- 1,638,552)
```

## Notes

A stretch goal for this project is to also (optionally) simulate decoherence, to allow
for more realistic simulations.

## License

This software is licensed under the MIT licence (see `LICENSE.md`)

Copyright (c) 2018 Adam Kelly


