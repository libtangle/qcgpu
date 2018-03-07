//! Gates and Gate Generation
//!
//! # Remarks
//!
//! Matrices are in column major format,
//! thus the matrix:
//!
//!  [a, b]
//!
//!  [c, d]
//!
//! Will be written:
//!  [a, c, b, d]
//!
//! All gates use the num_complex::Complex&lt;f32&gt;
//! datatype.

use arrayfire::{Array, Dim4, DType, identity_t};
use num_complex::Complex;
use kron;

/// Identity gate
///
/// [1, 0]
///
/// [0, 1]
pub fn id() -> Array {
    identity_t(Dim4::new(&[2, 2, 1, 1]), DType::C32)
}

/// Hadamard Gate
///
/// [0.70710678118, 0.70710678118]
///
/// [0.70710678118, -0.70710678118]
pub fn hadamard() -> Array {
    let coef: [Complex<f32>; 4] = [
        Complex::new(0.70710678118f32, 0.0), Complex::new(0.70710678118f32, 0.0),
        Complex::new(0.70710678118f32, 0.0), Complex::new(-0.70710678118f32, 0.0),
    ];

    return Array::new(&coef, Dim4::new(&[2, 2, 1, 1]))
}

/// Pauli X / NOT Gate
///
/// [0, 1]
///
/// [1, 0]
pub fn x() -> Array {
    let coef: [Complex<f32>; 4] = [
        Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
    ];

    return Array::new(&coef, Dim4::new(&[2, 2, 1, 1]))
}

/// Pauli Y Gate
///
/// [0, -i]
///
/// [i, 0]
pub fn y() -> Array {
    let coef: [Complex<f32>; 4] = [
        Complex::new(0.0, 0.0), Complex::new(0.0, 1.0),
        Complex::new(0.0, -1.0), Complex::new(0.0, 0.0),
    ];

    return Array::new(&coef, Dim4::new(&[2, 2, 1, 1]))
}

/// Pauli Z Gate
///
/// [1, 0]
///
/// [0, -1]
pub fn z() -> Array {
    let coef: [Complex<f32>; 4] = [
        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0),
    ];

    return Array::new(&coef, Dim4::new(&[2, 2, 1, 1]))
}

/// S Gate
///
/// [1, 0]
///
/// [0, i]
pub fn s() -> Array {
    let coef: [Complex<f32>; 4] = [
        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0), Complex::new(0.0, 1.0),
    ];

    return Array::new(&coef, Dim4::new(&[2, 2, 1, 1]))
}

/// T Gate
///
/// [1, 0]
///
/// [0,(1/sqrt(2)) * 1 + 1i]
pub fn t() -> Array {
    let coef: [Complex<f32>; 4] = [
        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0), Complex::new(0.7071067811865475244008443621048490393, 0.7071067811865475244008443621048490393),
    ];

    return Array::new(&coef, Dim4::new(&[2, 2, 1, 1]))
}

/// Generate a multi-qubit gate, applying a single
/// qubit gate to the target.
///
/// # Examples
///
/// ```
/// // Create a 3 qubit gate, applying a Pauli-X gate to
/// // the first qubit
/// generate_gate(x(), 3, 0);
/// ```
pub fn generate_gate(gate: Array, num_qubits: usize, target: i32) -> Array {
    let qubits_before = target;
    let qubits_after = num_qubits as i32 - target - 1;
    println!("Before: {} After: {}", qubits_before, qubits_after);

    if qubits_before == 0 && qubits_after == 0 {
        return gate
    } else if qubits_before == 0 {
        let after_arr = identity_t(Dim4::new(&[2 << (qubits_after - 1), 2 << (qubits_after - 1),1,1]), DType::C32);
        return kron::kron(&gate, &after_arr)
    } else if qubits_after == 0 {
        let before_arr = identity_t(Dim4::new(&[2 << (qubits_before - 1), 2 << (qubits_before - 1),1,1]), DType::C32);
        return kron::kron(&before_arr, &gate)
    } else {
        let before_arr = identity_t(Dim4::new(&[2 << (qubits_before - 1), 2 << (qubits_before - 1),1,1]), DType::C32);
        let mid_arr = kron::kron(&before_arr, &gate);
        let after_arr = identity_t(Dim4::new(&[2 << (qubits_after - 1), 2 << (qubits_after - 1),1,1]), DType::C32);
        return kron::kron(&mid_arr, &after_arr)
    }
}
