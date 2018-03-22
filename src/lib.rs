#![feature(test)]

extern crate ocl;
extern crate test;
extern crate num_complex;

mod kernel;
mod gates;
mod state;

pub use state::State;
pub use gates::Gate;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use num_complex::Complex32;
    use super::*;

    #[bench]
    fn qubits_20_not_all_gpu(b: &mut Bencher) {
    let x = Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(1.0, 0.0),
        c: Complex32::new(1.0, 0.0),
        d: Complex32::new(0.0, 0.0),
    };
        b.iter(|| {
            let mut state = State::new(20, 1);
            black_box(state.apply_all(x));
        });
    }

    #[bench]
    fn qubits_20_not_all_cpu(b: &mut Bencher) {
    let x = Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(1.0, 0.0),
        c: Complex32::new(1.0, 0.0),
        d: Complex32::new(0.0, 0.0),
    };
        b.iter(|| {
            let mut state = State::new(20, 0);
            black_box(state.apply_all(x));
        });
    }
}
