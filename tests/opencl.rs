extern crate qcgpu;

use qcgpu::Simulator;

fn create_simulator(n: u8) -> Simulator {
    let sim = Simulator::new_opencl(n);

    assert!(
            sim.is_ok(),
            "Error initializing OpenCL simulator"
        );

    return sim.unwrap();
}

#[test]
fn can_initialize_simulator() {
    for n in 1..25 {
        create_simulator(n);
    }
}

#[test]
fn can_apply_x_gate() {
    for n in 1..25 {
        let mut sim = create_simulator(n);

        for i in 0..n {
            assert!(
                sim.x(i).is_ok(),
                "Error applying pauli-x (not) gate to simulator"
            );
        }
    }
}

#[test]
fn can_apply_y_gate() {
    for n in 1..25 {
        let mut sim = create_simulator(n);

        for i in 0..n {
            assert!(
                sim.y(i).is_ok(),
                "Error applying pauli-y gate to simulator"
            );
        }
    }
}

#[test]
fn can_apply_z_gate() {
    for n in 1..25 {
        let mut sim = create_simulator(n);

        for i in 0..n {
            assert!(
                sim.z(i).is_ok(),
                "Error applying pauli-z gate to simulator"
            );
        }
    }
}

#[test]
fn can_apply_h_gate() {
    for n in 1..25 {
        let mut sim = create_simulator(n);

        for i in 0..n {
            assert!(
                sim.h(i).is_ok(),
                "Error applying hadamard gate to simulator"
            );
        }
    }
}

#[test]
fn can_apply_cx_gate() {
    for n in 2..25 {
        let mut sim = create_simulator(n);

        for i in 1..n {
            assert!(
                sim.cx(0, i).is_ok(),
                "Error applying controlled not gate to simulator"
            );
        }
    }
}
