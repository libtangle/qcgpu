extern crate qcgpu;

use qcgpu::Simulator;

#[test]
fn can_initialize_simulator() {
    for n in 1..25 {
        let sim = Simulator::new_opencl(n);

        assert!(
            sim.is_ok(),
            "Error initializing OpenCL simulator"
        );
    }
}
