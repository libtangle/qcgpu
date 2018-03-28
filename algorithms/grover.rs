extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::{negh, h, z, x};
use std::f32::consts::PI;
use std::env;

// Finding the number 11011
fn main() {
    let args: Vec<String> = env::args().collect();

    let num_arg = &args[1];
    let num = num_arg.parse::<i32>().unwrap();

    search(num);
}

fn search(n: i32) {
    let qubits_needed = (n as f32).log2().ceil() as u32;
    let num_amps = 2_i32.pow(qubits_needed);
    let num_amplifications_needed = ((num_amps as f32).sqrt() * PI / 4.0) as i32;

    println!("Searching for: {}", n);
    println!("Using: {} qubits", qubits_needed);
    println!("{} amplifications needed", num_amplifications_needed);

    let mut state = State::new(qubits_needed, 1);

    state.apply_all(h());

    for _ in 0..num_amplifications_needed {
        state.apply_grover_oracle(n);
        state.apply_grover_amplify();
    }

    println!("Measured: {:?}", state.measure_many(10000));
}
