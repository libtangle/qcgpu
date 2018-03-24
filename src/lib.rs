//! An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust
//!
//! See the code [here](https://github.com/QCGPU/QCGPU-rust).

#![feature(test)]

extern crate num_complex;
extern crate ocl;
extern crate rand;
extern crate test;

mod kernel;
mod state;
pub mod gates;

pub use state::State;
pub use gates::Gate;

#[cfg(test)]
mod tests {
    use gates::{h, x};
    use super::*;

    #[test]
    fn gate_application() {
        let mut state = State::new(1, 0);

        state.apply_gate(0, x());

        assert_eq!(state.measure(), 1);
    }
}
