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
        .warm_up_time(Duration::from_millis(500))
        .sample_size(100)
        .nresamples(25)
}

fn apply_single_gate(state: &mut State) {
    state.h(0);
}

fn apply_controlled_gate(state: &mut State) {
    state.cx(0, 1);
}

fn measure_n(state: &mut State, n: i32) {
    state.measure_many(n);
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

    /////////////////////////////////////////////////////////////////////////////////////
    // SINGLE MEASUREMENT
    /////////////////////////////////////////////////////////////////////////////////////
    let gpu_5_c = Fun::new("GPU", |b, _| {
        let mut state = State::new(5, 1);
        b.iter(|| measure_n(&mut state, 1));
    });
    let cpu_5_c = Fun::new("CPU", |b, _| {
        let mut state = State::new(5, 0);
        b.iter(|| measure_n(&mut state, 1));
    });
    let functions_5_c = vec![gpu_5_c, cpu_5_c];
    c.bench_functions("5 Qubits Single Measurement", functions_5_c, &1);

    let gpu_25_c = Fun::new("GPU", |b, _| {
        let mut state = State::new(25, 1);
        b.iter(|| measure_n(&mut state, 1));
    });
    let cpu_25_c = Fun::new("CPU", |b, _| {
        let mut state = State::new(25, 0);
        b.iter(|| measure_n(&mut state, 1));
    });
    let functions_25_c = vec![gpu_25_c, cpu_25_c];

    c.bench_functions("25 Qubits Single Measurement", functions_25_c, &1);

    /////////////////////////////////////////////////////////////////////////////////////
    // THOUSAND MEASUREMENTS
    /////////////////////////////////////////////////////////////////////////////////////
    let gpu_5_d = Fun::new("GPU", |b, _| {
        let mut state = State::new(5, 1);
        b.iter(|| measure_n(&mut state, 1000));
    });
    let cpu_5_d = Fun::new("CPU", |b, _| {
        let mut state = State::new(5, 0);
        b.iter(|| measure_n(&mut state, 1000));
    });
    let functions_5_d = vec![gpu_5_d, cpu_5_d];
    c.bench_functions("5 Qubits Thousand Measurements", functions_5_d, &1);

    let gpu_25_d = Fun::new("GPU", |b, _| {
        let mut state = State::new(25, 1);
        b.iter(|| measure_n(&mut state, 1000));
    });
    let cpu_25_d = Fun::new("CPU", |b, _| {
        let mut state = State::new(25, 0);
        b.iter(|| measure_n(&mut state, 1000));
    });
    let functions_25_d = vec![gpu_25_d, cpu_25_d];

    c.bench_functions("25 Qubits Thousand Measurements", functions_25_d, &1);

    ///////////////////////////////////////////////////////////////////////////////////
    // REGISTER CREATION
    ///////////////////////////////////////////////////////////////////////////////////
    let gpu_5_e = Fun::new("GPU", |b, _| {
        b.iter(|| State::new(5, 1));
    });
    let cpu_5_e = Fun::new("CPU", |b, _| {
         b.iter(|| State::new(5, 0));
    });
    let functions_5_e = vec![gpu_5_e, cpu_5_e];
    c.bench_functions("5 Qubits State Creation", functions_5_e, &1);

    let gpu_25_e = Fun::new("GPU", |b, _| {
         b.iter(|| State::new(25, 1));
    });
    let cpu_25_e = Fun::new("CPU", |b, _| {
         b.iter(|| State::new(25, 0));
    });
    let functions_25_e = vec![gpu_25_e, cpu_25_e];

    c.bench_functions("25 Qubits State Creation", functions_25_e, &1);
}


criterion_group!{
    name = benches;
    config = fast_benchmark();
    targets = benchmarks
}


criterion_main!(benches);
