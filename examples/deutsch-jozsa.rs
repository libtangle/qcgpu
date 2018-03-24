//! # Deutsch-Jozsa Algorithm
//!
//! This algorithm was the first to show that quantum computers could have a speedup over classical computers.
//!
//! Consider a function $f(x)$ which takes an input of an $n$-bit string $x$ and returns 0 or 1.
//!
//! Suppose that $f(x)$ is either a **constant** function that has the same value $c \in \{0, 1\},  \forall x$, or a **balanced** function, where the value is 0 for half of the inputs, and 1 for the other half.
//!
//! The Deutsch-Jozsa problem is to find whether $f$ is *constant* or *balanced*, in as few function evaluations as possible.
//!
//! Using classical computing, in the worst case, this requires $2^{n-1}+1$ function evaluations.
//! Using quantum computing, this can be done with just one function evaluation.
//!
//! The function $f$, to be used in a quantum computer, must be specified specified by an oracle circuit $U_f$ such that $U_f \lvert x \rangle = (-1)^{f(x)}\lvert x \rangle$.
//!
//! ## The Algorithm
//!
//! 1. Initialize $n$ qubits in the state $\lvert 0, \dots, 0\rangle$.
//! 2. Apply the Hadamard gate $H$ to each qubit.
//! 3. Apply the oracle circuit $U_f$.
//! 4. Apply the Hadamard gate $H$ to each qubit.
//! 5. Measure each qubit. Let $y = (y_1, \dots, y_n)$ be the list of measurement outcomes.
//!
//! From this procedure, we find that $f$ is constant if $y$ is the all zero string.

extern crate qcgpu;

use qcgpu::{Gate, State};
use qcgpu::gates::{h, x, z};

fn main() {
    // 3 qubits, f(x) = x_0 NOT x_1 x_2
    // Balanced
    let mut balanced_state = State::new(3, 1);

    balanced_state.apply_all(h());
    balanced_state.apply_gate(2, h());
    balanced_state.apply_gate(0, z());
    balanced_state.apply_controlled_gate(1, 2, x());
    balanced_state.apply_gate(2, h());
    balanced_state.apply_all(h());

    println!(
        "{}",
        if balanced_state.measure() == 0 {
            "constant"
        } else {
            "balanced"
        }
    );

    // 3 qubits, f(x) = 0
    // Constant
    let mut constant_state = State::new(3, 1);

    constant_state.apply_all(h());
    constant_state.apply_all(h());

    println!(
        "{}",
        if constant_state.measure() == 0 {
            "constant"
        } else {
            "balanced"
        }
    );
}
