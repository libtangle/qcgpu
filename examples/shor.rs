extern crate qcgpu;
use qcgpu::State;

/*
/**
 * Return x^y mod m
 */
fn pow_mod(x: i32, y: i32, m: i32) -> i32 {
    if y == 0 {
        return 1;
    }
    if y == 1 {
        return x;
    }

    let half_y = (y as f32 / 2f32).floor() as i32;
    let pow_half_y = pow_mod(x, half_y, m);
    let result = pow_half_y.pow(2) % m;

    if y % 2 == 1 {
        return (x * result) % m;
    } else {
        return result;
    }
}

fn power_factor(n: i32) -> i32 {
	let log2_n = (n as f32).ln() / 2f32.ln();
	let y = log2_n.floor();
	if (log2_n == y) {
    	return 2;
  	}

}

// A is a random number, n is the number being factored
fn compute_order(a: i32, n: i32) {
    let num_out_bits = ((n as f32).ln() / 2f32.ln()).ceil() as u32;
    let num_in_bits = (2 * num_out_bits) as u32;

    let input_range = 2i32.pow(num_in_bits);
    let output_range = 2i32.pow(num_out_bits);

    // Accuracy for continued fraction
    let accuracy = 1 / (2 * output_range * output_range);

    let out_bits_range = range::Range::new(0, num_out_bits - 1);
    let in_bits_range = range::Range::new(num_out_bits, num_out_bits + num_in_bits - 1);

    fn f(x: i32) -> i32 {
        pow_mod(a, x, n)
    }

    let f_0 = f(0);
}

fn factor(n: i32) -> i32 {
    // Check if even
    if n % 2 == 0 {
        return 2;
    }
    -1
}

*/
fn main() {
    let state = Sta:te:new(5);

    println!("{:?}", state.num_qubits);
}

