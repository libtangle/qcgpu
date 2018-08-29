extern crate failure;
extern crate num_complex;
extern crate ocl;
extern crate rand;

#[macro_use]
extern crate failure_derive;

pub mod backends;
pub mod error;
pub mod gate;
pub mod traits;

use backends::OpenCL;
pub use gate::{h, x, y, z, Gate};

use failure::Error;
use std::fmt;

#[derive(Debug)]
pub struct Simulator {
    backend: Box<traits::Backend>,
    num_qubits: u8,
}

impl Simulator {
    pub fn new_opencl(num_qubits: u8) -> Result<Simulator, Error> {
        let backend = OpenCL::new(num_qubits)?;

        Ok(Simulator {
            backend: Box::new(backend),
            num_qubits,
        })
    }

    pub fn apply_gate(&mut self, gate: Gate, target: u8) -> Result<(), Error> {
        self.backend.apply_gate(gate, target)
    }

    pub fn apply_all(&mut self, gate: Gate) -> Result<(), Error> {
        for i in 0..self.num_qubits {
            self.backend.apply_gate(gate, i)?
        }

        Ok(())
    }

    pub fn x(&mut self, target: u8) -> Result<(), Error> {
        self.backend.apply_gate(x(), target)
    }
    pub fn y(&mut self, target: u8) -> Result<(), Error> {
        self.backend.apply_gate(y(), target)
    }
    pub fn z(&mut self, target: u8) -> Result<(), Error> {
        self.backend.apply_gate(z(), target)
    }
    pub fn h(&mut self, target: u8) -> Result<(), Error> {
        self.backend.apply_gate(h(), target)
    }
    pub fn cx(&mut self, control: u8, target: u8) -> Result<(), Error> {
        self.backend.apply_controlled_gate(x(), control, target)
    }
    pub fn measure(&mut self) -> Result<u8, Error> {
        self.backend.measure()
    }
    pub fn num_qubits(&mut self) -> u8 {
        self.backend.num_qubits()
    }
}

impl fmt::Display for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.backend)
    }
}
