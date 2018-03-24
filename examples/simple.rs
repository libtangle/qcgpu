extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::{h, x};

fn main() {
    // New Quantum State with 10 qubits
    let mut state = State::new(21, 1);

    // Apply the gates
    state.apply_all(h());

    println!("{:?}", state.measure_many(1000));
}


