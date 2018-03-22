use num_complex::Complex32;

#[derive(Debug, Clone, Copy)]
pub struct Gate {
    pub a: Complex32,
    pub b: Complex32,
    pub c: Complex32,
    pub d: Complex32,
}
