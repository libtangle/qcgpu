use gate::Gate;

pub trait Backend {
    fn apply_gate(&mut self, gate: Gate, target: u8);
    fn apply_controlled_gate(&mut self, gate: Gate, control: u8, target: u8);
    fn measure(&self) -> u8;
}
