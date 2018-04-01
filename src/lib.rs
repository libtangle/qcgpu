#![deny(missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]

//! An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust
//!
//! This crate is a library with the aim to provide fast and effecient simulation of algorithms
//! for quantum computers, while also being easy to use and abstracting away the management of the
//! OpenCL Devices
//!
//! The best place to start with this library is with the [user guide](https://qcgpu.github.io/qcgpu/book),
//! along with the docs (you're looking at them) and the [source code](https://github.com/QCGPU/QCGPU-rust)
//!
//! ## Features
//! * Simulation of arbitrary quantum algorithms
//! * Optional simulation of decoherence
//! * Optimized for maximally entangled states
//! * Accelerated with GPUs, FPGAs and other OpenCL devices
//! * Example implementations of Grover, Deutsch-Jozsa, Bernstein-Vazirani and Shors algorithm
//! * Implements Hadamard, Pauli and phase gates, with support for arbitrary gates
//! * Support for arbitrary controlled gates


extern crate num_complex;
extern crate ocl;
extern crate rand;

mod kernel;
mod state;
mod utilities;
pub mod gates;

pub use state::State;
pub use gates::Gate;
pub use utilities::{gcd, get_width};
