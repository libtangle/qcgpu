use gate::Gate;
use std::collections::HashMap;

pub trait Backend {
    fn apply_gate(&mut self, gate: Gate, target: u8);
    fn apply_controlled_gate(&mut self, gate: Gate, control: u8, target: u8);
    fn measure(&mut self) -> u8;
    fn measure_many(&mut self, iters: u64) -> HashMap<u8, u64>;
}
