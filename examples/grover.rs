extern crate num_complex;
extern crate qcgpu;

use qcgpu::State;
use qcgpu::Gate;
use num_complex::Complex32;

// Grovers Search Algorithm
// This algorithm can speed up an unstructured searh problem quadratically.
//
// Suppose you have a large list of N items. With these, is one item with a *unique*
// property that we want to find.
//
// To find this item using classical algorithms, it would take on average N/2 steps,
// as the list is unstructured. Using a quantum algorithm, the item can be found in about
// sqrt(n) steps

// We need a way to provide the quantum computer to check if it is the correct
// item. We need to encode the list, in terms of a function f which returns
// f(x) = 1 for the item we are looking for, and f(x) = 0 for all other items.
//
// This function must be encoded into a unitary matrix, so it can be used on the
// quantum computer. It is commonly called an oracle.
// We choose a binary encoding of the items x,w \in {0, 1}^n; where w is the
// item we are looking for. We then define an oracle matrix U_f to act on any standard basis
// |x> by:
//
//      U_f |x> = (-1)^f(x) |x>
//
// The result of this is |x> for every item but the target, which is -|w>.
// On a bloch sphere, this corresponds with a reflection for the target item.

// The Circuit

// Grover search of 2 qubits, looking for the state 11

fn main() {
    let x = Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(1.0, 0.0),
        c: Complex32::new(1.0, 0.0),
        d: Complex32::new(0.0, 0.0),
    };

    let num_qubits = 2;
    let mut state = State::new(num_qubits, 1);

    state.apply_all(x);

    println!("{}", state);
}
