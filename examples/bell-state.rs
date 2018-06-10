extern crate qcgpu;

use qcgpu::Simulator;

fn main() {
    println!("Creating Bell State");

    let mut sim = Simulator::new_opencl(2);

    sim.h(0);
    sim.cx(0, 1);

    println!("Measurment Results:");
    println!("{}", sim.measure());
}
