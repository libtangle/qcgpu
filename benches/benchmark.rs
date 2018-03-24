#[macro_use]
extern crate criterion;
extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::{h, x};
use criterion::Criterion;

fn gpu_1() {
    // New Quantum State with 10 qubits
    let mut state = State::new(10, 1);

    // Apply the gates
    state.apply_all(h());

    for _i in 0..1 {
        state.measure();
    }
}

fn gpu_1000() {
    // New Quantum State with 10 qubits
    let mut state = State::new(10, 1);

    // Apply the gates
    state.apply_all(h());

    state.measure_many(1000);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1-GPU", |b| b.iter(|| gpu_1()));
    c.bench_function("1000-GPU", |b| b.iter(|| gpu_1000()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
