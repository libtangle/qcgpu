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

use arrayfire::{Array, Dim4, DType, Seq, identity_t, assign_seq, constant};
use num_complex::Complex;
use kron::kron;

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

/// S / Phase Gate
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
/// use qcgpu::gates::{generate_gate, x};
///
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
        return kron(&gate, &after_arr)
    } else if qubits_after == 0 {
        let before_arr = identity_t(Dim4::new(&[2 << (qubits_before - 1), 2 << (qubits_before - 1),1,1]), DType::C32);
        return kron(&before_arr, &gate)
    } else {
        let before_arr = identity_t(Dim4::new(&[2 << (qubits_before - 1), 2 << (qubits_before - 1),1,1]), DType::C32);
        let mid_arr = kron(&before_arr, &gate);
        let after_arr = identity_t(Dim4::new(&[2 << (qubits_after - 1), 2 << (qubits_after - 1),1,1]), DType::C32);
        return kron(&mid_arr, &after_arr)
    }
}

// Gets A Bit Of A Number
// Zero Indexed
//
// i.e. 4 === '100'. get_bit(4, 2) => '1'
fn get_bit(num: i32, n: i32) -> i32 {
    return (num >> n) & 1
}

// Toggle A Bit Of A Number
// Zero Indexed
//
// i.e. 4 === '100'. toggle_bit(4, 2) => '000'
fn toggle_bit(num: i32, bit: i32) -> i32 {
   num ^ 1 << bit
}

/// Generate a multi-qubit controlled NOT gate, applying a NOT
/// qubit gate to the target, if the control is 1.
///
/// # Examples
///
/// ```
/// use qcgpu::gates::generate_cnot;
///
/// // Create a 2 qubit gate, applying a Pauli-X gate to
/// // the second qubit if the first is 1. (Control of 0, target of 1)
/// generate_cnot(2, 0, 1);
/// // [4 4 1 1]
/// //   (1.0000,0.0000)          (0.0000,0.0000)          (0.0000,0.0000)          (0.0000,0.0000)
/// //   (0.0000,0.0000)          (1.0000,0.0000)          (0.0000,0.0000)          (0.0000,0.0000)
/// //   (0.0000,0.0000)          (0.0000,0.0000)          (0.0000,0.0000)          (1.0000,0.0000)
/// //   (0.0000,0.0000)          (0.0000,0.0000)          (1.0000,0.0000)          (0.0000,0.0000)
/// ```
pub fn generate_cnot(num_qubits: u32, control: i32, target: i32) -> Array {
    let mut arr = constant(Complex::new(0.0f32, 0.0), Dim4::new(&[2u64.pow(num_qubits), 2u64.pow(num_qubits), 1, 1]));
    let one = constant(Complex::new(1.0f32, 0.0), Dim4::new(&[1,1,1,1]));
    let c = num_qubits as i32 - 1 - control;
    let t = num_qubits as i32 - 1 - target;
    for i in 0..(2u64.pow(num_qubits) as i32) {
        if get_bit(i, c) == 1 {
            // This is a state that we want to apply the gate on
            // Input: i, Output: toggle_bit(i, t)
            let position = &[Seq::new(i, i, 1), Seq::new(toggle_bit(i, t), toggle_bit(i, t), 1)];
            arr = assign_seq(&arr, position, &one);
        } else {
            // Input: i, Output: i
            let position = &[Seq::new(i, i, 1), Seq::new(i, i, 1)];
            arr = assign_seq(&arr, position, &one);
        }
    }


    arr
}
