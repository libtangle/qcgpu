extern crate num_complex;
extern crate ocl;
extern crate rand;

pub mod gate;
pub mod traits;
pub mod backends;

use backends::OpenCL;
use gate::{Gate, h, x, y, z};

use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Simulator {
    backend: Box<traits::Backend>,
    num_qubits: u8,
}

impl Simulator {
    pub fn new_opencl(num_qubits: u8) -> Simulator {
        Simulator {
            backend: Box::new(OpenCL::new(num_qubits)),
            num_qubits
        }
    }

    pub fn apply_gate(&mut self, gate: Gate, target: u8) {
        self.backend.apply_gate(gate, target);
    }

    pub fn apply_all(&mut self, gate: Gate) {
        for i in 0..self.num_qubits { 
            self.backend.apply_gate(gate, i);
        }
    }


    pub fn x(&mut self, target: u8) {
        self.backend.apply_gate(x(), target);
    }

    pub fn y(&mut self, target: u8) {
        self.backend.apply_gate(y(), target);
    }
    
    pub fn z(&mut self, target: u8) {
        self.backend.apply_gate(z(), target);
    }

    pub fn h(&mut self, target: u8) {
        self.backend.apply_gate(h(), target);
    }

    pub fn cx(&mut self, control: u8, target: u8) {
        self.backend.apply_controlled_gate(x(), control, target);
    }

    pub fn measure(&self) -> u8 {
        self.backend.measure()
    }

    pub fn measure_many(&self, iters: u32) -> HashMap<u8, u32> {
        self.backend.measure_many(iters)
    }
}

impl fmt::Display for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.backend)
    }
}