use gate::Gate;
use std::fmt::{Debug, Display};
use std::collections::HashMap;

pub trait Backend: Debug + Display {
    fn apply_gate(&mut self, gate: Gate, target: u8);
    fn apply_controlled_gate(&mut self, gate: Gate, control: u8, target: u8);
    fn measure(&self) -> u8;
    fn measure_many(&self, iters: u32) -> HashMap<u8, u32> {
        let mut results = HashMap::new();

        for _ in 0..iters {
            let state = self.measure();
            let count = results.entry(state).or_insert(0);
            *count += 1;
        }

        results
    }
}
