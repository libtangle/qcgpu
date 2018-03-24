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
    use test::{Bencher};
    use gates::{h, x};
    use super::*;

    #[test]
    fn gate_application() {
        let mut state = State::new(1, 0);

        state.apply_gate(0, x());

        assert_eq!(state.measure(), 1);
    }

    #[bench]
    fn ghz_state_gpu(b: &mut Bencher) {
        b.iter(|| {
            // New Quantum State with 25 qubits
            let mut state = State::new(25, 1);

            // Print the hardware that the simulation will run on
            print!("Running On: ");
            state.info();

            // Apply the gates
            state.apply_gate(0, h());
            state.apply_gate(1, h());
            state.apply_gate(2, x());

            state.apply_controlled_gate(1, 2, x());
            state.apply_controlled_gate(0, 2, x());

            state.apply_all(h());

            // XXX Measurement
            state.apply_all(h());

            println!("State Vector: {}", state);
            println!("Probabilities: {:?}", state.get_probabilities());
            println!("Measured: {}", state.measure());
        });
    }

    #[bench]
    fn ghz_state_cpu(b: &mut Bencher) {
        b.iter(|| {
            // New Quantum State with 25 qubits
            let mut state = State::new(25, 1);

            // Print the hardware that the simulation will run on
            print!("Running On: ");
            state.info();

            // Apply the gates
            state.apply_gate(0, h());
            state.apply_gate(1, h());
            state.apply_gate(2, x());

            state.apply_controlled_gate(1, 2, x());
            state.apply_controlled_gate(0, 2, x());

            state.apply_all(h());

            // XXX Measurement
            state.apply_all(h());

            println!("State Vector: {}", state);
            println!("Probabilities: {:?}", state.get_probabilities());
            println!("Measured: {}", state.measure());
        });
    }
}
