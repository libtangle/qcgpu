#![feature(test)]

extern crate test;
extern crate num_complex;
extern crate arrayfire;

use arrayfire::{print, set_backend, info, Backend};

mod state;

fn main() {
    set_backend(Backend::OPENCL);
    info();

    let state = state::QState::from_bit_string("|1>");
    print(&state.amplitude);
}


#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use arrayfire::{set_backend, Backend};
    use super::*;

    #[bench]
    fn qubits_16_cpu(b: &mut Bencher) {
        set_backend(Backend::CPU);
        b.iter(|| {
            black_box(state::QState::new(16));
        });
    }

    #[bench]
    fn qubits_16_opencl(b: &mut Bencher) {
        set_backend(Backend::OPENCL);
        b.iter(|| {
            black_box(state::QState::new(16));
        });
    }

    #[bench]
    fn qubits_16_from_string_cpu(b: &mut Bencher) {
        set_backend(Backend::CPU);
        b.iter(|| {
            black_box(state::QState::new(16));
        });
    }

    #[bench]
    fn qubits_16_from_string_opencl(b: &mut Bencher) {
        set_backend(Backend::OPENCL);
        b.iter(|| {
            black_box(state::QState::new(16));
        });
    }
}
