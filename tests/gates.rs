extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::x;

#[test]
fn hadamard() {
    for i in 1..18 {
        let mut state = State::new(i, 0);

        let mut h_state = State::new(i, 0);
        h_state.h(0);
        h_state.h(0);

        assert_eq!(state.measure_many(100), h_state.measure_many(100));
    }
}

#[test]
fn not() {
    for i in 1..18 {
        let mut state = State::new(i, 0);
        state.x(0);

        assert_eq!(state.measure(), 1);
    }
}

#[test]
fn apply_all() {
    for i in 1..18 {
        let mut state = State::new(i, 0);
        state.apply_all(x());

        assert_eq!(state.measure(), 2_i32.pow(i) - 1);
    }
}

#[test]
fn controlled_not() {
    // Test by creating a bell state
    let mut state = State::new(2, 0);
    state.h(0);
    state.cx(0, 1);

    let measurements = state.measure_many(1000);
    assert!(!measurements.contains_key("10") && !measurements.contains_key("01"));
}

#[test]
fn toffoli() {
    // Test by creating a bell state
    let mut state = State::new(3, 0);
    state.h(0);
    state.h(1);
    state.toffoli(0, 1, 2);

    let measurements = state.measure_many(1000);
    assert!(
        !measurements.contains_key("100") && !measurements.contains_key("110")
            && !measurements.contains_key("011") && !measurements.contains_key("101")
    );
}

#[test]
fn swap() {
    let mut state = State::new(2, 0);
    state.h(0);
    state.swap(0, 1);

    let measurements = state.measure_many(1000);
    assert!(!measurements.contains_key("01") && !measurements.contains_key("11"));
}
