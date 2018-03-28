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
        .sample_size(10)
        .nresamples(10)
}

fn apply_single_gate(state: &mut State) {
    state.h(0);
}

fn apply_controlled_gate(state: &mut State) {
    state.cx(0, 1);
}

fn cnot_first(num_qubits: u32, backend: usize) {
    let mut state = State::new(num_qubits, backend);
    state.cx(0, 1);
    state.measure();
}

fn benchmarks(c: &mut Criterion) {
    ///////////////////////////////////////////////////////////////////////////////////
    // SINGLE GATE APPLICATION
    ///////////////////////////////////////////////////////////////////////////////////
    let gpu_5 = Fun::new("GPU", |b, _| {
        let mut state = State::new(5, 1);
        b.iter(|| apply_single_gate(&mut state));
    });
    let cpu_5 = Fun::new("CPU", |b, _| {
        let mut state = State::new(5, 0);
        b.iter(|| apply_single_gate(&mut state));
    });
    let functions_5 = vec![gpu_5, cpu_5];
    c.bench_functions("5 Qubits Single Gate Application", functions_5, &1);

    let gpu_25 = Fun::new("GPU", |b, _| {
        let mut state = State::new(25, 1);
        b.iter(|| apply_single_gate(&mut state));
    });
    let cpu_25 = Fun::new("CPU", |b, _| {
        let mut state = State::new(25, 0);
        b.iter(|| apply_single_gate(&mut state));
    });
    let functions_25 = vec![gpu_25, cpu_25];


    c.bench_functions("25 Qubits Single Gate Application", functions_25, &1);

    /////////////////////////////////////////////////////////////////////////////////////
    // CONTROLLED GATE APPLICATION
    /////////////////////////////////////////////////////////////////////////////////////
    let gpu_5_b = Fun::new("GPU", |b, _| {
        let mut state = State::new(5, 1);
        b.iter(|| apply_controlled_gate(&mut state));
    });
    let cpu_5_b = Fun::new("CPU", |b, _| {
        let mut state = State::new(5, 0);
        b.iter(|| apply_controlled_gate(&mut state));
    });
    let functions_5_b = vec![gpu_5_b, cpu_5_b];
    c.bench_functions("5 Qubits Controlled Gate Application", functions_5_b, &1);

    let gpu_25_b = Fun::new("GPU", |b, _| {
        let mut state = State::new(25, 1);
        b.iter(|| apply_controlled_gate(&mut state));
    });
    let cpu_25_b= Fun::new("CPU", |b, _| {
        let mut state = State::new(25, 0);
        b.iter(|| apply_controlled_gate(&mut state));
    });
    let functions_25_b = vec![gpu_25_b, cpu_25_b];


    c.bench_functions("25 Qubits Controlled Gate Application", functions_25_b, &1);
}


criterion_group!{
    name = benches;
    config = fast_benchmark();
    targets = benchmarks
}


criterion_main!(benches);
