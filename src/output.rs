//! Measurement Output Utilities
//!
//! Formats the results of multiple measurements,
//! Single measurements and partial measurements.

use num_complex::Complex32;
use std::fmt;
use std::collections::HashMap;


pub fn get_counts(results: Vec<i32>, num_qubits: u32) -> HashMap<String, i32> {
    let mut num_results = HashMap::new();

    for result in results {
        let state = format!("{:0width$b}", result, width = num_qubits as usize);
        let count = num_results.entry(state).or_insert(0);
        *count += 1;
    }

    num_results
}
