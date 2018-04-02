extern crate qcgpu;

use qcgpu::State;

#[test]
fn register_creation() {
    for i in 1..18 {
        // 18 is the fastest that will run in a quick time for travis
        let mut state = State::new(i, 0);
        assert_eq!(state.measure(), 0);
    }
}

#[test]
fn register_from_bitstring() {
    let mut state_1 = State::from_bit_string("|00>", 0);
    assert_eq!(state_1.measure(), 0);

    let mut state_2 = State::from_bit_string("|11>", 0);
    assert_eq!(state_2.measure(), 3);

    let mut state_3 = State::from_bit_string("|10110>", 0);
    assert_eq!(state_3.measure(), 22);

    let mut state_4 = State::from_bit_string("|0000001111101110>", 0);
    assert_eq!(state_4.measure(), 1006);
}

#[test]
fn num_qubits() {
    for i in 1..18 {
        // 18 is the fastest that will run in a quick time for travis
        let mut state = State::new(i, 0);
        assert_eq!(state.num_qubits, i);
    }
}


#[test]
fn add_scratch() {
    for i in 1..16 {
        // 18 is the fastest that will run in a quick time for travis
        let mut state = State::new(i, 0);
        state.add_scratch(2);
        assert_eq!(state.num_qubits, i + 2);
    }
}

