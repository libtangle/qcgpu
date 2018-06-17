use gate::Gate;
use std::fmt::{Debug, Display};
use failure::Error;

pub trait Backend: Debug + Display {
    fn num_qubits(&self) -> u8;
    fn apply_gate(&mut self, gate: Gate, target: u8) -> Result<(), Error>;
    fn apply_controlled_gate(&mut self, gate: Gate, control: u8, target: u8) -> Result<(), Error>;
    fn measure_qubit(&mut self, target: u8) -> Result<u8, Error>;

    fn measure(&mut self) -> Result<u8, Error> {
        let mut result = 0;
        for i in 0..self.num_qubits() {
            let bit_mask = 1 << i;
            if self.measure_qubit(i)? == 1 {
                // 1, set the bit in result
                result = result | bit_mask
            } else {
                // 0, clear the bit in result
                result = result & (!bit_mask)
            }
        }

        Ok(result)
    }
}
