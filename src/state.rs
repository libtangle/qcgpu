use ocl::enums::DeviceInfo::Type;
use ocl::{Buffer, MemFlags, ProQue};
use num_complex::Complex32;
use std::fmt;
use std::collections::HashMap;
use rand::random;

use kernel::KERNEL;
use gates::Gate;
use gates::{h, s, t, x, y, z};

/// Representation of a quantum register

#[derive(Debug)]
pub struct State {
    pub buffer: Buffer<Complex32>,
    pub pro_que: ProQue,
    pub num_amps: usize,
    pub num_qubits: u32,
    pub backend: usize,
}

impl State {
    /// Create a new quantum register, with a given number of qubits.
    /// The backend is the OpenCL ID of the accelerator to use.
    ///
    /// The register will be initialized in the state |00...0>
    ///
    /// ```rust
    /// # extern crate qcgpu;
    /// let state = qcgpu::State::new(2, 1);
    /// ```
    pub fn new(num_qubits: u32, backend: usize) -> State {
        let num_amps = 2_u32.pow(num_qubits) as usize;

        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(backend)
            .dims(num_amps)
            .build()
            .expect("Error Building ProQue");

        let mut source = vec![Complex32::new(0.0, 0.0); ocl_pq.dims().to_len()];
        source[0] = Complex32::new(1.0, 0.0);
        // let source = vec![1.0f32, 0.0, 0.0, 0.0];

        // create a temporary vector with the source buffer
        let source_buffer = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write().copy_host_ptr())
            .len(num_amps)
            .copy_host_slice(&source)
            .build()
            .expect("Source Buffer");

        State {
            buffer: source_buffer,
            pro_que: ocl_pq,
            num_amps,
            num_qubits,
            backend,
        }
    }

    /// Create a new quantum register, starting in the
    /// State given. The backend is the OpenCL ID of the
    /// accelerator to use.
    ///
    /// ```rust
    /// # extern crate qcgpu;
    /// let state = qcgpu::State::from_bit_string("|00>", 1);
    /// ```
    pub fn from_bit_string(bit_string: &str, backend: usize) -> State {
        let bits = bit_string.to_string().replace("|", "").replace(">", "");
        let num_amps = 2 << (bits.len() - 1) as usize;

        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(backend)
            .dims(num_amps)
            .build()
            .expect("Error Building ProQue");

        let mut source = vec![Complex32::new(0.0, 0.0); ocl_pq.dims().to_len()];
        let value = i32::from_str_radix(bits.as_str(), 2).unwrap();
        source[value as usize] = Complex32::new(1.0, 0.0);
        // let source = vec![1.0f32, 0.0, 0.0, 0.0];

        // create a temporary vector with the source buffer
        let source_buffer = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write().copy_host_ptr())
            .len(num_amps)
            .copy_host_slice(&source)
            .build()
            .expect("Source Buffer");

        State {
            buffer: source_buffer,
            pro_que: ocl_pq,
            num_amps,
            num_qubits: bits.len() as u32,
            backend,
        }
    }

    /// Apply a gate to the target qubit
    pub fn apply_gate(&mut self, target: i32, gate: Gate) {
        // create a temporary vector with the source buffer
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer().unwrap();

        let apply = self.pro_que
            .kernel_builder("apply_gate")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .arg(target)
            .arg(gate.a)
            .arg(gate.b)
            .arg(gate.c)
            .arg(gate.d)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        self.buffer = result_buffer;
    }

    /// Apply a gate to every qubit in the register
    pub fn apply_all(&mut self, gate: Gate) {
        for i in 0..self.num_qubits as i32 {
            self.apply_gate(i, gate);
        }
    }

    /// Apply a gate to the register if the control qubit is 1.
    pub fn apply_controlled_gate(&mut self, control: i32, target: i32, gate: Gate) {
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer().unwrap();

        let apply = self.pro_que
            .kernel_builder("apply_controlled_gate")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .arg(control)
            .arg(target)
            .arg(gate.a)
            .arg(gate.b)
            .arg(gate.c)
            .arg(gate.d)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        self.buffer = result_buffer;
    }

    /// Return the probabilities of each outcome.
    ///
    /// The probabilitity of a state a|x> being measured
    /// is |a|^2.
    pub fn get_probabilities(&mut self) -> Vec<f32> {
        let result_buffer: Buffer<f32> = self.pro_que.create_buffer().unwrap();

        let apply = self.pro_que
            .kernel_builder("calculate_probabilities")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        let mut vec_result = vec![0.0f32; self.num_amps];
        result_buffer.read(&mut vec_result).enq().unwrap();

        vec_result
    }

    /// Return the state vector of the quantum register
    pub fn get_amplitudes(&mut self) -> Vec<Complex32> {
        let mut vec_result = vec![Complex32::new(0.0, 0.0); self.num_amps];
        self.buffer.read(&mut vec_result).enq().unwrap();

        vec_result
    }

    /// Measure the quantum register, returning the measured result
    pub fn measure(&mut self) -> i32 {
        let probabilities = self.get_probabilities();

        let mut key = random::<f32>();
        if key > 1.0 {
            key %= 1.0;
        }

        let mut i = 0;
        while i < probabilities.len() {
            key -= probabilities[i];
            if key <= 0.0 {
                break;
            }
            i += 1;
        }

        i as i32
    }

    /// Preform multiple measurements, returning the results
    /// as a HashMap, with the key as the result and the value as teh
    /// number of times that result was measured
    pub fn measure_many(&mut self, num_iterations: i32) -> HashMap<String, i32> {
        let probabilities = self.get_probabilities();
        let mut num_results = HashMap::new();

        for _ in 0..num_iterations {
            let mut key = random::<f32>();
            if key > 1.0 {
                key %= 1.0;
            }

            let mut i = 0;
            while i < probabilities.len() {
                key -= probabilities[i];
                if key <= 0.0 {
                    break;
                }
                i += 1;
            }
            let state = format!("{:0width$b}", i, width = self.num_qubits as usize);
            let count = num_results.entry(state).or_insert(0);
            *count += 1;
        }

        num_results
    }

    /// Add qubits to the register. The qubits are initialized to zero.
    /// This should be used as scratch space.
    pub fn add_scratch(&mut self, num_scratch: u32) {
        let num_amps = 2_u32.pow(self.num_qubits + num_scratch) as usize;
        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(self.backend)
            .dims(num_amps)
            .build()
            .expect("Error Building ProQue");

        let mut amps = self.get_amplitudes();
        amps.extend(vec![
            Complex32::new(0.0, 0.0);
            ocl_pq.dims().to_len() - self.pro_que.dims().to_len()
        ]);

        // create a temporary vector with the source buffer
        let source_buffer = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write().copy_host_ptr())
            .len(num_amps)
            .copy_host_slice(&amps)
            .build()
            .expect("Source Buffer");

        self.buffer = source_buffer;
        self.pro_que = ocl_pq;
        self.num_amps = num_amps;
        self.num_qubits += num_scratch;
    }

    /// Measure the scratch qubits. The measurement is discarded, and
    /// the register size is reduced by `num_to_measure` qubits.
    pub fn measure_scratch(&mut self, num_to_measure: u32) {
        let num_amps = 2_u32.pow(self.num_qubits - num_to_measure) as usize;
        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(self.backend)
            .dims(num_amps)
            .build()
            .expect("Error Building ProQue");

        let mut amps = self.get_amplitudes();
        amps.truncate(num_amps);

        // create a temporary vector with the source buffer
        let source_buffer = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write().copy_host_ptr())
            .len(num_amps)
            .copy_host_slice(&amps)
            .build()
            .expect("Source Buffer");

        self.buffer = source_buffer;
        self.pro_que = ocl_pq;
        self.num_amps = num_amps;
        self.num_qubits -= num_to_measure;
    }

    /// Print Information About The Device
    pub fn info(&self) {
        println!(
            "Device type: {:?}",
            self.pro_que.device().info(Type).unwrap()
        )
    }

    /* Ease Of Access / shorthand Functions*/

    /// Hadamard Gate
    /// Shorthand Method
    ///
    /// Equivilent to `state.apply_gate(target, h());`
    pub fn h(&mut self, target: i32) {
        self.apply_gate(target, h());
    }

    /// S Gate
    /// Shorthand Method
    ///
    /// Equivilent to `state.apply_gate(target, s());`
    pub fn s(&mut self, target: i32) {
        self.apply_gate(target, s());
    }

    /// T Gate
    /// Shorthand Method
    ///
    /// Equivilent to `state.apply_gate(target, t());`
    pub fn t(&mut self, target: i32) {
        self.apply_gate(target, t());
    }

    /// Pauli X Gate
    /// Shorthand Method
    ///
    /// Equivilent to `state.apply_gate(target, x());`
    pub fn x(&mut self, target: i32) {
        self.apply_gate(target, x());
    }

    /// Pauli Y Gate
    /// Shorthand Method
    ///
    /// Equivilent to `state.apply_gate(target, y());`
    pub fn y(&mut self, target: i32) {
        self.apply_gate(target, y());
    }

    /// Pauli Z Gate
    /// Shorthand Method
    ///
    /// Equivilent to `state.apply_gate(target, z());`
    pub fn z(&mut self, target: i32) {
        self.apply_gate(target, z());
    }

    /// Controlled Not Gate S
    /// Shorthand method
    ///
    /// Equivilent to `state.apply_controlled_gate(control, target, x());`
    pub fn cx(&mut self, control: i32, target: i32) {
        self.apply_controlled_gate(control, target, x());
    }

    /// Toffoli (Controlled-Controlled-NOT gate)
    /// Shorthand method
    pub fn toffoli(&mut self, control1: i32, control2: i32, target: i32) {
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer().unwrap();

        let gate = x();

        let apply = self.pro_que
            .kernel_builder("apply_controlled_controlled_gate")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .arg(control1)
            .arg(control2)
            .arg(target)
            .arg(gate.a)
            .arg(gate.b)
            .arg(gate.c)
            .arg(gate.d)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        self.buffer = result_buffer;
    }

    /// Swap two qubits in the register
    pub fn swap(&mut self, first_qubit: i32, second_qubit: i32) {
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer().unwrap();

        let apply = self.pro_que
            .kernel_builder("swap")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .arg(first_qubit)
            .arg(second_qubit)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        self.buffer = result_buffer;
    }

    /// Caclulates f(a) = x^a mod n.
    pub fn pow_mod(&mut self, x: i32, n: i32, input_width: i32, output_width: i32) {
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer().unwrap();

        let apply = self.pro_que
            .kernel_builder("apply_pow_mod")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .arg(x)
            .arg(n)
            .arg(input_width)
            .arg(output_width)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        self.buffer = result_buffer;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        let mut vec_result = vec![Complex32::new(0.0, 0.0); self.num_amps];
        self.buffer.read(&mut vec_result).enq().unwrap();

        for (idx, item) in vec_result.iter().enumerate() {
            if first {
                write!(f, "[{idx}]: {}", item, idx = idx).unwrap();
                first = false;
            } else {
                write!(f, ", [{idx}]: {}", item, idx = idx).unwrap();
            }
        }

        Ok(())
    }
}
