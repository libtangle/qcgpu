use num_complex::Complex;
use arrayfire::*;
use rand::random;
use kron;
use gates;

pub struct QState {
    pub num_qubits: usize,
    pub amplitude: Array,
}


fn get(a: &Array, i: i32, j: i32) -> Array {
    let seqs = &[Seq::new(i, i, 1), Seq::new(j, j, 1)];
    return index(a, seqs);
}

impl QState {
    pub fn new(n: usize) -> QState {
        let amps = identity_t(Dim4::new(&[2 << (n - 1), 1, 1, 1]), DType::C32);
        //let one = constant(Complex::new(1.0f32, 1.0), Dim4::new(&[1, 1, 1, 1]));
        //let seqs = &[Seq::new(0.0, 1.0, 1.0), Seq::new(0.0, 1.0, 1.0)];

        QState {
            num_qubits: n,
            amplitude: amps,
        }
    }

    pub fn from_bit_string(bit_string: &str) -> QState {
        let bits = bit_string.to_string().replace("|", "").replace(">", "");

        let value = i32::from_str_radix(bits.as_str(), 2).unwrap();

        let mut amps = constant(
            Complex::new(0.0f32, 0.0),
            Dim4::new(&[2 << (bits.len() - 1), 1, 1, 1]),
        );

        let position = &[Seq::new(value, value, 1)]; // begin n end n step
        let one = constant(Complex::new(1.0f32, 0.0), Dim4::new(&[1, 1, 1, 1]));
        amps = assign_seq(&amps, position, &one);

        return QState {
            num_qubits: bits.len(),
            amplitude: amps,
        };
    }

    pub fn kron(&self, state: QState) -> QState {
        return QState {
            num_qubits: self.num_qubits + state.num_qubits,
            amplitude: kron::kron(&self.amplitude, &state.amplitude),
        };
    }


    pub fn apply_gate(&mut self, gate: &Array, target: i32) {
        // Make a new copy of amps
        let mut amps = constant(Complex::new(0.0f32, 0.0), Dim4::new(&[2 << (self.num_qubits - 1), 1, 1, 1]));

        for state in 0..2i32.pow(self.num_qubits as u32) as i32 {
            // Could Check Here If The Current Amp is zero, if so, skip.
            let zero_state = state & (!(1 << target)); // Clear the target bit
            let one_state = state | (1 << target); // Set the target bit

            let bit_val = if ((1 << target) & state) > 0 { 1 } else { 0 };

            let position_zero = &[Seq::new(zero_state, zero_state, 1)];
            let position_one = &[Seq::new(one_state, one_state, 1)];

            let amp = get(&self.amplitude, state, 0);

            amps = assign_seq(&amps, position_zero, &(&get(&amps, zero_state, 0) + &(&get(gate, bit_val, 0) * &amp)));
            amps = assign_seq(&amps, position_one, &(&get(&amps, one_state, 0) + &(&get(gate, bit_val, 1) * &amp)));
        }

        self.amplitude = amps;
    }



    pub fn cnot(&mut self, control: i32, target: i32) {
        let full_gate = gates::generate_cnot(self.num_qubits, control, target);
        self.amplitude = matmul(&full_gate, &self.amplitude, MatProp::NONE, MatProp::NONE);
    }

    pub fn apply_all(&mut self, gate: &Array) {
        for i in 0..self.num_qubits as i32 {
            self.apply_gate(gate, i);
        }
    }

    pub fn measure(&mut self) -> i32 {
        let probabilities = pow(&self.amplitude, &2, true);

        let mut key = random::<f32>();
        if key > 1.0 {
            key = key % 1.0;
        }

        let mut i = 0;
        while i < probabilities.elements() as i32 {
            let mut vec = vec![0.0f32; 1];

            let seqs = &[Seq::new(i, i, 1)];
            index(&probabilities, seqs).host(&mut vec);

            key = key - vec[0];

            if key <= 0.0 {
                return i;
            }

            i = i + 1;
        }

        i
    }

    pub fn partial_measure(&mut self) -> Array {
        unimplemented!()
    }
}
