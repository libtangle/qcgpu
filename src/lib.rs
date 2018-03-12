#![feature(test)]

//! An Open Source, High Performance & GPU Accelerated, Quantum Computer Simulator in Rust
extern crate arrayfire;
extern crate num_complex;
extern crate test;
extern crate rand;

pub mod state;
pub mod kron;
pub mod gates;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use arrayfire::{identity_t, set_backend, Array, Backend, DType, Dim4};
    use num_complex::Complex;
    use super::*;

    #[test]
    fn cnot_0_1_opencl() {
        let generated_cnot = gates::generate_cnot(2, 0, 1);

        let values: [Complex<f32>; 16] = [
            Complex::new(1.0f32, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        let dims: Dim4 = Dim4::new(&[4, 4, 1, 1]);
        let spec_cnot = Array::new(&values, dims);

        let mut vals1: Vec<Complex<f32>> = vec![Complex::new(0.0f32, 0.0); 16];
        let mut vals2: Vec<Complex<f32>> = vec![Complex::new(0.0f32, 0.0); 16];

        generated_cnot.host::<Complex<f32>>(&mut vals1);
        spec_cnot.host::<Complex<f32>>(&mut vals2);

        assert_eq!(vals1, vals2);
    }

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
            black_box(gates::generate_gate(gates::hadamard(), 8, 3));
        });
    }

    #[bench]
    fn qubits_8_hadamard_gate_opencl(b: &mut Bencher) {
        set_backend(Backend::OPENCL);
        b.iter(|| {
            black_box(gates::generate_gate(gates::hadamard(), 8, 3));
        });
    }

    #[bench]
    fn kron_cpu(b: &mut Bencher) {
        set_backend(Backend::CPU);
        let arr_1 = identity_t(Dim4::new(&[4, 4, 1, 1]), DType::C32);
        let arr_2 = identity_t(Dim4::new(&[2, 2, 1, 1]), DType::C32);

        b.iter(|| {
            black_box(kron::kron(&arr_1, &arr_2));
        });
    }

    #[bench]
    fn kron_a_opencl(b: &mut Bencher) {
        set_backend(Backend::OPENCL);
        let arr_1 = identity_t(Dim4::new(&[4, 4, 1, 1]), DType::C32);
        let arr_2 = identity_t(Dim4::new(&[2, 2, 1, 1]), DType::C32);

        b.iter(|| {
            black_box(kron::kron(&arr_1, &arr_2));
        });
    }
}
