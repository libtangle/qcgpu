extern crate qcgpu;

use qcgpu::{State};
use qcgpu::gates::{x, h};

fn main() {
    let mut state = State::new(2, 1);
    print!("Running On: ");
    state.info();
    state.apply_gate(0, h());
    state.apply_controlled_gate(0, 1, x());
    println!("State Vector: {}", state);
    println!("Probabilities: {:?}", state.get_probabilities());
    print!("Measured: ");
    for i in 0..10 {
        print!("{}, ", state.measure());
    }
}
