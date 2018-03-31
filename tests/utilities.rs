extern crate qcgpu;

use qcgpu::get_width;

#[test]
fn get_width_test() {
    assert_eq!(get_width(1), 1);
    assert_eq!(get_width(3), 2);
    assert_eq!(get_width(7), 3);
    assert_eq!(get_width(8), 4);
}
