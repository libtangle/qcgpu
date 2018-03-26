//! # Bernstein-Vazirani Algorithm
//!
//! This algorithm finds a hidden integer $a \in \{ 0, 1\}^n$ from
//! an oracle $f_a$ which returns a bit $a \cdot x \equiv \sum_i a_i x_i \mod 2$
//! for an input $x \in \{0,1\}^n$.
//!
//! A classical oracle returns $f_a(x) = a \dot x \mod 2$, while the quantum oracle
//! must be queried with superpositions of input $x$'s.
//!
//! To solve this problem classically, the hidden integer can be found by checking the
//! oracle with the inputs $x = 1,2,/dots,2^i,2^{n-1}$, where each
//! query reveals the $i$th bit of $a$ ($a_i$).
//! This is the optimal classical solution, and is O(n). Using a quantum oracle and the
//! Bernstein-Vazirani algorithm, $a$ can be found with just one query to the oracle.
//!
//! ## The Algorithm
//!
//! 1. Initialize $n$ qubits in the state $\lvert 0, \dots, 0\rangle$.
//! 2. Apply the Hadamard gate $H$ to each qubit.
//! 3. Apply the inner product oracle.
//! 4. Apply the Hadamard gate $H$ to each qubit.
//! 5. Measure the register
//!
//! From this procedure, we find that the registers measured value is equal to that of
//! the original hidden integer.

extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::h;

fn main() {
    let num_qubits = 16; // Number of qubits to use
    let a = 101; // Hidden integer, bitstring is 1100101

    // You should also make sure that a is representable with $n$ qubits,
    // by settings a as $a mod 2^n$.

    // Bernstein-Vazirani algorithm
    let mut state = State::new(num_qubits, 1); // New quantum register, using the GPU.

    // Apply a hadamard gate to each qubit
    state.apply_all(h());

    // Apply the inner products oracle
    for i in 0..num_qubits {
        if a & (1 << i) != 0 {
            state.z(i as i32);
        }
        // Otherwise should apply identity gate, but computationally this doens't change the state.
    }

    // Apply hadamard gates before measuring
    state.apply_all(h());

    println!("Measurement Results: {:?}", state.measure_many(1000));
    // Measurement Results: {"0000000001100101": 1000}
}
