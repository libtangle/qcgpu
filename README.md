# QCGPU

> An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust

[![Build Status](https://travis-ci.org/QCGPU/qcgpu-rust.svg?branch=master)](https://travis-ci.org/QCGPU/qcgpu-rust)

* Uses OpenCL kernels.
* Runs Cross Platform.

## Prerequisites:
* OpenCL (Ensure that an OpenCL library is installed for your platform and that `clinfo` or some other diagnostic command will run). 
* Rust (install [here](https://www.rustup.rs)). Please use a nightly build.

## Usage

First, add the crate to `cargo.toml`

```toml
[dependencies]
qcgpu = { git = "https://github.com/QCGPU/QCGPU-rust" }
```

Then just use the crate!

```rust
extern crate qcgpu;

fn main() {
    let mut state = qcgpu::State::new(2, 0);
    state.apply_gate(0, qcgpu::gates::x());

    println!("Measured: {}", state.measure());
    // 1
}

```

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


