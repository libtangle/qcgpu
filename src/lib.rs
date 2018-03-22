#![feature(test)]

extern crate ocl;
extern crate test;

mod kernel;
mod gates;
mod state;

pub use state::State;
pub use gates::Gate;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use super::*;

    #[bench]
    fn qubits_15_not_all_gpu(b: &mut Bencher) {
        let x = Gate {
            a: 0.0,
            b: 1.0,
            c: 1.0,
            d: 0.0,
        };
        b.iter(|| {
            let mut state = State::new(15, 1);
            black_box(state.apply_all(x));
        });
    }

    #[bench]
    fn qubits_15_not_all_cpu(b: &mut Bencher) {
        let x = Gate {
            a: 0.0,
            b: 1.0,
            c: 1.0,
            d: 0.0,
        };
        b.iter(|| {
            let mut state = State::new(15, 0);
            black_box(state.apply_all(x));
        });
    }
}
