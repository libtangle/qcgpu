//! # Super Dense Coding
//!
//! If Alice and Bob share a pair of entangled qubits, then Alice can encode two classical bits into her one entangled qubit,
//! send it to Bob, and Bob can decode it with the help of his entangled qubit.

extern crate qcgpu;

use qcgpu::{State};
use qcgpu::gates::{x, h, z};

fn superdense(input: &str) -> i32 {
    let mut state = State::new(2, 0);
    let input_str = String::from(input);

    // Prepare the bell state
    state.apply_gate(0, h());
    state.apply_controlled_gate(0, 1, x());

    // Alice prepares her qubit
    let alice = 1;
    if input_str.get(0..1) == Some("1") {
        state.apply_gate(alice, z());
    }
    if input_str.get(1..2) == Some("1") {
        state.apply_gate(alice, x());
    }

    println!("\nState after Alice prepares her qubit: \n{}", state);

    // Alice sends her qubit to Bob
    let bob = 0;
    state.apply_controlled_gate(alice, bob, x());
    state.apply_gate(alice, h());

    println!("\nState after Bob receives Alice's qubit and 'decodes' it: \n{}", state);

    return state.measure()
}

fn main() {
    use std::io;

    println!("Two bit string to send:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let result = superdense(input.as_str());
            println!("\nDecoded string is: {}", result);
        }
        Err(error) => println!("error: {}", error),
    }
}
