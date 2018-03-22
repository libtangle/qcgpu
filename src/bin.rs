extern crate qcgpu;
extern crate num_complex;

use qcgpu::{Gate, State};
use num_complex::Complex32;

fn main() {
    let x = Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(1.0, 0.0),
        c: Complex32::new(1.0, 0.0),
        d: Complex32::new(0.0, 0.0),
    };

    let h = Gate {
        a: Complex32::new(0.70710678118, 0.0),
        b: Complex32::new(0.70710678118, 0.0),
        c: Complex32::new(0.70710678118, 0.0),
        d: Complex32::new(-0.70710678118, 0.0),
    };

    let y = Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(0.0, -1.0),
        c: Complex32::new(0.0, 1.0),
        d: Complex32::new(0.0, 0.0),
    };

    let mut state = State::new(1, 1);
    state.info();
state.apply_gate(0, x);
    state.apply_gate(0, y);
    //state.apply_all(x);
    //state.apply_controlled_gate(0, 1, x);
    state.print();
}
