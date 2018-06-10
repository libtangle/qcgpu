extern crate num_complex;
extern crate ocl;
extern crate rand;

pub mod gate;
pub mod traits;
pub mod backends;

use backends::OpenCL;
use gate::{h, x};

pub struct Simulator {
    backend: Box<traits::Backend>,
}

impl Simulator {
    pub fn new_opencl(num_qubits: u8) -> Simulator {
        Simulator {
            backend: Box::new(OpenCL::new(num_qubits))
        }
    }

    pub fn x(&mut self, target: u8) {
        self.backend.apply_gate(x(), target);
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
}