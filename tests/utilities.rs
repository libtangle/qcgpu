extern crate qcgpu;

use qcgpu::{gcd, get_width};

#[test]
fn get_width_test() {
    assert_eq!(get_width(1), 1);
    assert_eq!(get_width(3), 2);
    assert_eq!(get_width(7), 3);
    assert_eq!(get_width(8), 4);
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(23, 7), 1);
    assert_eq!(gcd(4, 2), 2);
    assert_eq!(gcd(102, 4), 2);
    assert_eq!(gcd(22937552, 432784), 432784);
}

