use num_complex::Complex32;
use std::fmt;
use std::f32::consts::FRAC_1_SQRT_2;

pub struct Gate {
    pub a: Complex32,
    pub b: Complex32,
    pub c: Complex32,
    pub d: Complex32,
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{}, {}], [{}, {}]]", self.a, self.b, self.c, self.d)
    }
}

/// Hadamard Gate
///
/// [0.70710678118, 0.70710678118]
///
/// [0.70710678118, -0.70710678118]
#[inline]
pub fn h() -> Gate {
    Gate {
        a: Complex32::new(FRAC_1_SQRT_2, 0.0),
        b: Complex32::new(FRAC_1_SQRT_2, 0.0),
        c: Complex32::new(FRAC_1_SQRT_2, 0.0),
        d: Complex32::new(-FRAC_1_SQRT_2, 0.0),
    }
}

/// Pauli X / NOT Gate
///
/// [0, 1]
///
/// [1, 0]
#[inline]
pub fn x() -> Gate {
    Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(1.0, 0.0),
        c: Complex32::new(1.0, 0.0),
        d: Complex32::new(0.0, 0.0),
    }
}