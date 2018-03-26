//! Preperation of the 3 qubit GHZ state

extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::h;

fn main() {
    // New Quantum State with 3 qubits
    let mut state = State::new(25, 1);

    // Print the hardware that the simulation will run on
    print!("Running On: ");
    state.info();

    // Apply the gates
    state.h(0);
    state.h(1);
    state.x(2);

    state.cx(1, 2);
    state.cx(0, 2);

    state.apply_all(h());

    // XXX Measurement
    state.apply_all(h());

    // println!("State Vector: {}", state);
    //println!("Probabilities: {:?}", state.get_probabilities());
    println!("Measured: {}", state.measure());
}
