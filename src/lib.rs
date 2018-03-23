//! An Open Source, High Performance & Hardware Accelerated, Quantum Computer Simulator in Rust
//!
//! See the code [here](https://github.com/QCGPU/QCGPU-rust).

#![feature(test)]

extern crate num_complex;
extern crate ocl;
extern crate test;
extern crate rand;

mod kernel;
mod state;
pub mod gates;

pub use state::State;
pub use gates::Gate;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use gates::{h, x};
    use super::*;

    #[bench]
    fn ghz_state_gpu(b: &mut Bencher) {
        b.iter(|| {
            // New Quantum State with 3 qubits
            let mut state = State::new(20, 1);

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
            // New Quantum State with 3 qubits
            let mut state = State::new(20, 1);

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
