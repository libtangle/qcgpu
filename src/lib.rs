//#![warn(missing-debug-implementations, missing-docs, trivial-casts, trivial-numeric-casts, unused-extern-crates, unused-import-braces, unused-qualifications, unused-results)]

#![deny(missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]
//! An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust
//!
//! See the code [here](https://github.com/QCGPU/QCGPU-rust).
//!
//! Decoherence can be enabled as a feature `decoherence`. The amount of decoherence can then be
//! set through the method `set_decoherence`

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
