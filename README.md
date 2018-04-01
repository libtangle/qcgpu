# QCGPU

> An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust

[![Build Status](https://travis-ci.org/QCGPU/qcgpu-rust.svg?branch=master)](https://travis-ci.org/QCGPU/qcgpu-rust)

**Features:**

* Uses OpenCL kernels
* Runs cross platform
* Simulation of arbitrary quantum algorithms
* Optional simulation of decoherence
* Optimized for maximally entangled states
* Accelerated with GPUs
* Includes Example Implementations of Grover's algorithm, the Deutsch-Jozsa algorithm, the Bernstein-Vazirani algorithm and Shors algorithm
* Implements Hadamard, Pauli and phase gates, with support for arbitrary single qubit gate creation
* Support for controlled gates along with the toffoli and swap gate
* Support for measuring a state many times quickly

## Prerequisites:
* OpenCL (Ensure that an OpenCL library is installed for your platform and that `clinfo` or some other diagnostic command will run). 
* Rust (install [here](https://www.rustup.rs)). Please use a nightly build.

If you wish to run the benchmarks, you must also install libquantum and qiskit.

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

Running on an amazon EC2 p3.2xlarge instance, with a 25GB General Purpose SSD (GP2)

```rust
5 Qubits Single Gate Application/GPU
                        time:   [204.39 us 214.76 us 225.98 us]
5 Qubits Single Gate Application/CPU
                        time:   [157.85 us 160.23 us 162.47 us]
25 Qubits Single Gate Application/GPU
                        time:   [103.50 ms 105.64 ms 108.39 ms]
25 Qubits Single Gate Application/CPU
                        time:   [248.81 ms 249.93 ms 250.53 ms]
5 Qubits Controlled Gate Application/GPU
                        time:   [205.55 us 209.50 us 214.34 us]
5 Qubits Controlled Gate Application/CPU
                        time:   [158.28 us 159.56 us 161.78 us]
25 Qubits Controlled Gate Application/GPU
                        time:   [106.13 ms 108.15 ms 110.50 ms]
25 Qubits Controlled Gate Application/CPU
                        time:   [246.37 ms 247.91 ms 248.89 ms]
```

## License

This software is licensed under the MIT licence (see `LICENSE.md`)

Copyright (c) 2018 Adam Kelly


