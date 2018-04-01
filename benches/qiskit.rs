#[macro_use]
extern crate criterion;
extern crate qcgpu;

use std::time::Duration;
use std::process::Command;
use criterion::Criterion;
use qcgpu::State;
use qcgpu::gates::h;

// Criterion struct for really fast benchmarks
fn fast_benchmark() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(250))
        .sample_size(5)
        .nresamples(2)
}

fn hadamard_all_command() -> Command {
    let mut command = Command::new("python3");
    command.arg("benches/qiskit/hadamard_all.py");
    command
}

fn benchmarks(c: &mut Criterion) {
    c.bench_program_over_inputs(
        "qiskit Hadamard All And Measure 1000",
        hadamard_all_command,
        &[2, 4, 6, 8],
    );

    c.bench_function_over_inputs(
        "qcgpu Hadamard All And Measure 1000",
        |b, i| {
            b.iter(|| {
                let mut state = State::new(**i as u32, 1);
                state.apply_all(h());
                state.measure_many(1000);
            })
        },
        &[2, 4, 6, 8],
    );
}

criterion_group!{
    name = benches;
    config = fast_benchmark();
    targets = benchmarks
}

criterion_main!(benches);
