#[macro_use]
extern crate criterion;
extern crate qcgpu;

use criterion::Criterion;
use qcgpu::State;

// Criterion struct for really fast benchmarks
fn fast_benchmark() -> Criterion {
    Criterion::default().sample_size(10).nresamples(2)
}

fn benchmarks(c: &mut Criterion) {
    ///////////////////////////////////////////////////////////////////////////////////
    // SINGLE GATE APPLICATION
    ///////////////////////////////////////////////////////////////////////////////////

    c.bench_function_over_inputs(
        "Single Gate Application",
        |b, &size| {
            let mut state = State::new(size, 0);
            b.iter(|| state.h(0));
        },
        vec![
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
        ],
    );

    ///////////////////////////////////////////////////////////////////////////////////
    // 50 GATE APPLICATIONS
    ///////////////////////////////////////////////////////////////////////////////////

    c.bench_function_over_inputs(
        "50 Gate Applications",
        |b, &size| {
            let mut state = State::new(size, 0);
            b.iter(|| {
                for _ in 0..50 {
                    state.x(0);
                }
            });
        },
        vec![
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
        ],
    );

    /////////////////////////////////////////////////////////////////////////////////////
    // CONTROLLED GATE APPLICATION
    /////////////////////////////////////////////////////////////////////////////////////

    c.bench_function_over_inputs(
        "Controlled Gate Application",
        |b, &size| {
            let mut state = State::new(size, 0);
            b.iter(|| state.cx(0, 1));
        },
        vec![
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
        ],
    );

    /////////////////////////////////////////////////////////////////////////////////////
    // SINGLE MEASUREMENT
    /////////////////////////////////////////////////////////////////////////////////////

    c.bench_function_over_inputs(
        "Single Measurement",
        |b, &size| {
            let mut state = State::new(size, 0);
            b.iter(|| state.measure());
        },
        vec![
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
        ],
    );

    /////////////////////////////////////////////////////////////////////////////////////
    // THOUSAND MEASUREMENTS
    /////////////////////////////////////////////////////////////////////////////////////

    c.bench_function_over_inputs(
        "Thousand Measurements",
        |b, &size| {
            let mut state = State::new(size, 0);
            b.iter(|| state.measure_many(1000));
        },
        vec![
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
        ],
    );

    ///////////////////////////////////////////////////////////////////////////////////
    // REGISTER CREATION
    ///////////////////////////////////////////////////////////////////////////////////

    c.bench_function_over_inputs(
        "State Creation",
        |b, &size| {
            b.iter(|| State::new(size, 0));
        },
        vec![
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
        ],
    );
}

criterion_group!{
    name = benches;
    config = fast_benchmark();
    targets = benchmarks
}

criterion_main!(benches);
