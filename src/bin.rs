extern crate qcgpu;
extern crate arrayfire;

use qcgpu::state::QState;
use qcgpu::gates::x;
use arrayfire::print;

fn main() {
    let mut state = QState::from_bit_string("|00>");
    print(&state.amplitude);
    state.apply_gate(x(), 0);
    state.cnot(0, 1);

    print(&state.amplitude);
}
