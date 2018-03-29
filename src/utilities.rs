use num_complex::Complex32;
use std::fmt;
use std::f32::consts::FRAC_1_SQRT_2;

/// Get the number of qubits needed to represent a number.
///
/// Equivilent to ceil(log2(n))s
pub fn get_width(n: i32) -> i32 {
    ((n as f32) + 1.0).log2().ceil() as i32
}
