<h1 align="center">QCGPU</h1>
<div align="center">Open Source, High Performance & Hardware Accelerated - Quantum Computer Simulation Library</div>


<div align="center">
	<a href="https://qcgpu.github.io/qcgpu-rust/book/getting-started.html">Getting Started</a>
    |
    <a href="https://qcgpu.github.io/qcgpu-rust/book/">User Guide</a>
    |
    <a href="https://qcgpu.github.io/qcgpu-rust/doc/qcgpu/index.html">Documentation</a>
    |
    <a href="https://qcgpu.github.io/qcgpu-rust/book/algorithms/algorithms.html">Algorithms</a>
</div>

<div align="center">
	<a href="https://travis-ci.org/QCGPU/qcgpu-rust">
        <img src="https://travis-ci.org/QCGPU/qcgpu-rust.svg?branch=master" alt="Travis-CI">
    </a>
    <a href="https://crates.io/crates/qcgpu">
        <img src="https://img.shields.io/crates/v/qcgpu.svg" alt="Crates.io">
    </a>
</div>

The goal of QCGPU is to provide a library for the simulation of 
quantum computers that is fast, efficient and portable.

QCGPU is written in Rust and uses OpenCL to run code on the CPU, GPU or any other OpenCL supported devices.
This library is meant to be used both independently and alongside 
established tools for example compilers or more general and high level frameworks.
If you are interested in using QCGPU with IBM's [QISKit](http://qiskit.org/) 
framework or [QISKit ACQUA](https://qiskit.org/acqua/), please see 
the repository [qiskit-addon-qcgpu](https://github.com/qcgpu/qiskit-addon-qcgpu).

## Prerequisites

To use this library, you will need two things:

* OpenCL (Ensure that an OpenCL library is installed for your platform and that `clinfo` or some other diagnostic command will run).
* Rust (install [here](https://www.rustup.rs)). Please use a nightly build.

## Usage

First, add the crate to `cargo.toml`

```toml
[dependencies]
qcgpu = 1.0.0 
```

Then use the crate!

```rust
extern crate qcgpu;
use qcgpu::State;

fn main() {
    let mut state = State::new(2, 0).unwrap();
    state.x(0);

    println!("Measured: {}", state.measure().unwrap());
    // 1
}
```

## License

This software is licensed under the MIT licence (see `LICENSE`)

Copyright (c) 2018 Adam Kelly


