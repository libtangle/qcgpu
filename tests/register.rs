extern crate qcgpu;

use qcgpu::State;

#[test]
fn hadamard_gate() {
    let mut state = State::new(1, 0);
    state.h(0);
    state.h(0);

    assert_eq!(state.measure(), 0);
}
