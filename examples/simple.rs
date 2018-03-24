extern crate qcgpu;

use std::env;
use qcgpu::State;
use qcgpu::gates::{h, x};

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_qubits = &args[1];

    let mut state = State::new(num_qubits.parse::<u32>().unwrap(), 1);

    // Apply the gates
    state.apply_all(h());

    println!("Measured: {:?}", state.measure());
}
