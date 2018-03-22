extern crate qcgpu;

use qcgpu::{Gate, State};

fn main() {
    let x = Gate {
        a: 0.0,
        b: 1.0,
        c: 1.0,
        d: 0.0,
    };

    let h = Gate {
        a: 1.0 / 2f32.sqrt(),
        b: 1.0 / 2f32.sqrt(),
        c: 1.0 / 2f32.sqrt(),
        d: -1.0 / 2f32.sqrt(),
    };

    let mut state = State::new(2, 1);
    state.info();

    state.apply_gate(0, h);
    state.apply_controlled_gate(0, 1, x);
    state.print();
}
