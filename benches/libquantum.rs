#[macro_use]
extern crate criterion;
extern crate libquantum;
extern crate qcgpu;

use std::time::Duration;
use criterion::{Criterion, Fun};
use qcgpu::State;
use qcgpu::gates::h;

// Criterion struct for really fast benchmarks
fn fast_benchmark() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(220))
        .sample_size(10)
        .nresamples(5)
}

fn benchmarks(c: &mut Criterion) {
    ///////////////////////////////////////////////////////////////////////////////////
    // REGISTER CREATION
    ///////////////////////////////////////////////////////////////////////////////////

    let libquantum_create = Fun::new("libquantum", |b, i| {
        let arg = *i as usize;
        b.iter(|| libquantum::QuReg::new(arg, 0))
    });
    let qcgpu_create = Fun::new("QCGPU", |b, i| {
        let arg = *i as u32;
        b.iter(|| qcgpu::State::new(arg, 1))
    });

    let functions_create = vec![libquantum_create, qcgpu_create];

    c.bench_functions("Create Register", functions_create, 26);

    /////////////////////////////////////////////////////////////////////////////////////
    // SINGLE GATE APPLICATION
    /////////////////////////////////////////////////////////////////////////////////////

    let libquantum_single_application = Fun::new("libquantum", |b, i| {
        let arg = *i as usize;
        let mut state = libquantum::QuReg::new(arg, 0);
        b.iter(|| state.hadamard(0))
    });
    let qcgpu_single_application = Fun::new("QCGPU", |b, i| {
        let arg = *i as u32;
        let mut state = State::new(arg, 1);
        b.iter(|| state.h(0))
    });

    let functions_single_application =
        vec![libquantum_single_application, qcgpu_single_application];

    c.bench_functions("Single Gate Application", functions_single_application, 22);

    /////////////////////////////////////////////////////////////////////////////////////
    // REGISTER GATE APPLICATION
    /////////////////////////////////////////////////////////////////////////////////////

    let libquantum_register_application = Fun::new("libquantum", |b, i| {
        let arg = *i as usize;
        let mut state = libquantum::QuReg::new(arg, 0);
        b.iter(|| {
            for index in 0..arg {
                state.hadamard(index);
            }
        })
    });
    let qcgpu_register_application = Fun::new("QCGPU", |b, i| {
        let arg = *i as u32;
        let mut state = State::new(arg, 1);
        b.iter(|| state.apply_all(h()))
    });

    let functions_register_application =
        vec![libquantum_register_application, qcgpu_register_application];

    c.bench_functions(
        "Register Gate Application",
        functions_register_application,
        22,
    );

    /////////////////////////////////////////////////////////////////////////////////////
    // CONTROLLED GATE APPLICATION
    /////////////////////////////////////////////////////////////////////////////////////

    let libquantum_controlled_application = Fun::new("libquantum", |b, i| {
        let arg = *i as usize;
        let mut state = libquantum::QuReg::new(arg, 0);
        // Apply a hadamard to the whole state
        for index in 0..arg {
            state.hadamard(index);
        }
        b.iter(|| state.cnot(0, arg - 1))
    });
    let qcgpu_controlled_application = Fun::new("QCGPU", |b, i| {
        let arg = *i as u32;
        let mut state = State::new(arg, 1);
        state.apply_all(h());
        b.iter(|| state.cx(0, arg as i32 - 1))
    });

    let functions_controlled_application = vec![
        libquantum_controlled_application,
        qcgpu_controlled_application,
    ];

    c.bench_functions(
        "Controlled Gate Application",
        functions_controlled_application,
        22,
    );
}

criterion_group!{
    name = benches;
    config = fast_benchmark();
    targets = benchmarks
}

criterion_main!(benches);
