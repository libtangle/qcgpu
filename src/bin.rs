extern crate qcgpu;

use qcgpu::{Gate, State};

fn main() {
    let x = Gate {
        a: 0.0,
        b: 1.0,
        c: 1.0,
        d: 0.0,
    };

    let mut state = State::new(5, 1);
    state.info();

    state.apply_all(x);
    state.print();
}
