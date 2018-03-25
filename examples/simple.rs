extern crate qcgpu;

use std::env;
use qcgpu::State;
use qcgpu::gates::{h, x};

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_q = &args[1];
    let num_qubits = num_q.parse::<u32>().unwrap();

    let mut state = State::new(num_qubits, 1);

    // Apply the gates
    state.apply_gate(0, h());
    state.apply_controlled_gate(0, (num_qubits - 1) as i32, x());
    println!("{:?}", state.measure_many(1000));
}
