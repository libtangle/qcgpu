use num_complex::Complex32;
use std::fmt;

#[derive(Debug, Clone, Copy)]
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
