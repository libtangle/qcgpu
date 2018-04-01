//! # Grovers Algorithm
//!
//! Given an unstructured set $N = \{a_1, a_2,\dots,a_n\}$, find
//! a given element $a_i \in N$.
//!
//! This implementation looks for a given number $target$ in the set $\{0,1,\dots, reg_width\}$
//!
//! See https://cs.uwaterloo.ca/~watrous/LectureNotes.html

extern crate qcgpu;
extern crate rand;

use qcgpu::State;
use std::f32::consts::PI;

fn main() {
    let mut state = State::new(8, 1);

    let target = 5;
    let reg_width = 3;

    let num_inversions = ((PI / 4.0) * ((1 << reg_width) as f32).sqrt()) as i32;

    state.x(reg_width);

    for i in 0..(reg_width + 1) {
        state.h(i);
    }

    for _ in 0..(num_inversions) {
        iteration(&mut state, target, reg_width);
    }

    state.h(reg_width);

    println!("Measured: {:?}", state.measure_first(reg_width, 1000));
}

fn oracle(state: &mut State, target: i32, reg_width: i32) {
    for i in 0..reg_width {
        if get_bit(target, i) == 0 {
            state.x(i);
        }
    }

    state.toffoli(0, 1, reg_width + 1);

    let mut i = 1;
    while i < reg_width {
        state.toffoli(i, reg_width + i, reg_width + i + 1);
        i += 1;
    }

    state.cx(reg_width + i, reg_width);

    i = reg_width - 1;
    while i > 0 {
        state.toffoli(i, reg_width + i, reg_width + i + 1);
        i -= 1;
    }

    state.toffoli(0, 1, reg_width + 1);

    for i in 0..reg_width {
        if get_bit(target, i) == 0 {
            state.x(i);
        }
    }
}

fn inversion(state: &mut State, reg_width: i32) {
    for i in 0..reg_width {
        state.x(i);
    }

    state.h(reg_width - 1);

    if reg_width == 3 {
        state.toffoli(0, 1, 2);
    } else {
        state.toffoli(0, 1, reg_width + 1);

        let mut i = 1;
        while i < reg_width {
            state.toffoli(i, reg_width + i, reg_width + i + 1);
            i += 1;
        }

        state.cx(reg_width + i, reg_width - 1);

        i = reg_width - 2;
        while i > 0 {
            state.toffoli(i, reg_width + i, reg_width + i + 1);
            i -= 1;
        }

        state.toffoli(0, 1, reg_width + 1);
    }

    state.h(reg_width - 1);

    for i in 0..reg_width {
        state.x(i);
    }
}

fn iteration(state: &mut State, target: i32, reg_width: i32) {
    oracle(state, target, reg_width);

    for i in 0..reg_width {
        state.h(i);
    }

    inversion(state, reg_width);

    for i in 0..reg_width {
        state.h(i);
    }
}

/// Get the value of a bit
fn get_bit(number: i32, n: i32) -> i32 {
    if number & (1 << n) != 0 {
        return 1;
    }
    0
}
