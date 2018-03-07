#![crate_name = "qcgpu"]
#![feature(test)]

//! An Open Source, High Performance & GPU Accelerated, Quantum Computer Simulator in Rust

extern crate test;
extern crate num_complex;
extern crate arrayfire;

pub mod state;
pub mod kron;
pub mod gates;


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

    #[bench]
    fn qubits_8_hadamard_gate_cpu(b: &mut Bencher) {
        set_backend(Backend::CPU);
        b.iter(|| {
            black_box(gates::generate_gate(&gates::hadamard(), 8, 3));
        });
    }

    #[bench]
    fn qubits_8_hadamard_gate_opencl(b: &mut Bencher) {
        set_backend(Backend::OPENCL);
        b.iter(|| {
            black_box(gates::generate_gate(&gates::hadamard(), 8, 3));
        });
    }

    #[bench]
    fn kron_cpu(b: &mut Bencher) {
        set_backend(Backend::CPU);
        let arr_1 = identity_t(Dim4::new(&[4,4,1,1]), DType::C32);
        let arr_2 = identity_t(Dim4::new(&[2,2,1,1]), DType::C32);

        b.iter(|| {
            black_box(kron::kron(&arr_1, &arr_2));
        });
    }
     #[bench]
    fn kron_a_opencl(b: &mut Bencher) {
        set_backend(Backend::OPENCL);
        let arr_1 = identity_t(Dim4::new(&[4,4,1,1]), DType::C32);
        let arr_2 = identity_t(Dim4::new(&[2,2,1,1]), DType::C32);

        b.iter(|| {
            black_box();
        });
    }
}
