//! An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust
//!
//! See the code [here](https://github.com/QCGPU/QCGPU-rust).
//!
//! ## Tutorials
//!
//! - [Getting Started with QCGPU](./getting_started.html)

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
    use super::*;

    #[test]
    fn gate_application() {
        let mut state = State::new(1, 0);

        state.x(0);

        assert_eq!(state.measure(), 1);
    }

    #[test]
    fn swap_gate() {
        let mut state = State::new(2, 0);

        state.x(0);

        let before_probabilities = state.get_probabilities();

        // Applying a swap gate twice should
        // keep the register the same
        state.swap(0,1);
        state.swap(0,1);

        let after_probabilities = state.get_probabilities();

        assert_eq!(before_probabilities, after_probabilities);
    }
}
