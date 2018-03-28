//! # Grovers Algorithm
//!
//! Given an unstructured set $N = \{a_1, a_2,\dots,a_n\}$, find
//! a given element $a_i \in N$.
//!
//! This implementation looks for a given number $a$ in the set $\{0,1,\dots,\lceil \log2 (a) \rceil\}$
//!
//! ## The Algorithm
//!
//! (pseudo code)
//!
//! ```pseudo
//! Initialize $\lceil \log2 (a) \rceil\$ qubits to the 1 state.
//! ```
//!
//! See https://cs.uwaterloo.ca/~watrous/LectureNotes.html

extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::{h, x, z};

// Finding the number 10 from 2 qubits
fn main() {
    let mut state = State::new(2, 1);
    state.apply_all(h());
    state.s(0);
    state.h(1);
    state.cx(0, 1);
    state.h(1);
    state.s(0);
    state.apply_all(h());
    state.apply_all(x());
    state.h(1);
    state.cx(0, 1);
    state.h(1);
    state.apply_all(h());
    state.apply_all(x());

    println!("Measured: {:?}", state.measure_many(100_000));
}
