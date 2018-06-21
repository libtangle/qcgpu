//! # Bell State / EPR Pair
//!
//! The Bell State, also known as the EPR pair (after Einstein, Podosky and Rosen)
//! is the simplest example of entanglement.
//!
//! The Bell State is defined as the maximally entangled quantum state of two qubits.
extern crate qcgpu;

use qcgpu::Simulator;

fn main() {
    println!("Creating Bell State");

    let mut sim = Simulator::new_opencl(2).unwrap();

    sim.h(0).unwrap();
    sim.cx(0, 1).unwrap();

    println!("Measurement Results:");
    println!("{}", sim.measure().unwrap());
}
