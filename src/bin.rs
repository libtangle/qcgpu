extern crate arrayfire;
extern crate qcgpu;

use qcgpu::state::QState;
use qcgpu::gates::{x, hadamard};
use arrayfire::{info, print, set_backend, Backend};

fn main() {
    set_backend(Backend::CPU);
    info();

    let mut state = QState::from_bit_string("|00");
    //print(&state.amplitude);
    state.apply_all(&hadamard());

    //let cnot = generate_cnot(15, 0, 1);
    print(&state.amplitude);
   // state.apply_gate(hadamard(), 0);
    //state.cnot(0, 16);

    let mut zero = 0;
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;

    for _ in 0..100000 {
        match state.measure() {
            0 => zero = zero + 1,
            1 => one = one + 1,
            2 => two = two + 1,
            3 => three = three + 1,
            _ => (),
        }
    }

    println!("0: {}, 1: {}, 2: {}, 3: {}", zero, one, two, three);
    // 0: 24954, 1: 24971, 2: 25048, 3: 25027
}
