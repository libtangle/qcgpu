#[macro_use]
extern crate criterion;
extern crate qcgpu;

use std::time::Duration;
use criterion::{Criterion, Fun};
use qcgpu::State;
use qcgpu::gates::h;

// Criterion struct for really fast benchmarks
fn fast_benchmark() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(250))
        .sample_size(5)
        .nresamples(50)
}

fn hadamard_all(backend: i32, num_qubits: u32) {
    let mut state = State::new(num_qubits, backend);
    state.apply_all(0, h());
    state.measure();
}

fn tests(c: &mut Criterion) {
    let gpu_5 = Fun::new("GPU", |b, _| b.iter(|| hadamard_all(5, 1)));
    let cpu_5 = Fun::new("CPU", |b, _| b.iter(|| hadamard_all(5, 0)));
    let functions_5 = vec![gpu_5, cpu_5];

    let gpu_25 = Fun::new("GPU", |b, _| b.iter(|| hadamard_all(25, 1)));
    let cpu_25 = Fun::new("CPU", |b, _| b.iter(|| hadamard_all(25, 0)));
    let functions_25 = vec![gpu_25, cpu_25];

    c.bench_functions("5 Qubits Simple", functions_5, &1);
    c.bench_functions("25 Qubits Simple", functions_25, &1);
}

criterion_group!{
    name = benches;
    config = fast_benchmark();
    targets = tests
}
criterion_main!(benches);
