//! An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust
//!
//! See the code [here](https://github.com/QCGPU/QCGPU-rust).
//!
//! ## Tutorials
//!
//! - [Getting Started with QCGPU](./getting_started.html)

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
