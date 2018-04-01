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
    /// The OpenCL Buffer for the state vector
    pub buffer: Buffer<Complex32>,
    pro_que: ProQue,
    /// Number of amplitudes stored in the state vector
    pub num_amps: usize,
    /// Number of qubits in the register
    pub num_qubits: u32,
    /// The OpenCL Backend used. Use the method `info()` to get the devices identifier
    pub backend: usize,

    /// The amount of decoherence
    #[cfg(feature = "decoherence")]
    pub decoherence: f32,
}

impl State {
    /// Create a new quantum register, with a given number of qubits.
    /// The backend is the OpenCL ID of the accelerator to use.
    ///
    /// The register will be initialized in the state |00...0>
    ///
    /// ```rust
    /// # extern crate qcgpu;
    /// let state = qcgpu::State::new(2,0);
    /// ```
    pub fn new(num_qubits: u32, backend: usize) -> State {
        let num_amps = 2_u32.pow(num_qubits) as usize;

        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(backend)
            .dims(num_amps)
            .build()
            .expect("Error Building ProQue");

        // create a temporary vector with the source buffer
        let source_buffer: Buffer<Complex32> = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write())
            .len(num_amps)
            .build()
            .expect("Source Buffer");

        let apply = ocl_pq
            .kernel_builder("initalize_register")
            .arg(&source_buffer)
            .arg(0)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        State {
            buffer: source_buffer,
            pro_que: ocl_pq,
            num_amps,
            num_qubits,
            backend,

            #[cfg(feature = "decoherence")]
            decoherence: 0.0,
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
        let num_amps = 2 << (bits.len() - 1);

        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(backend)
            .dims(num_amps)
            .build()
            .expect("Error Building ProQue");

        let value = i32::from_str_radix(bits.as_str(), 2).unwrap();

        // create a temporary vector with the source buffer
        let source_buffer: Buffer<Complex32> = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write())
            .len(num_amps)
            .build()
            .expect("Source Buffer");

        let apply = ocl_pq
            .kernel_builder("initalize_register")
            .arg(&source_buffer)
            .arg(value)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        State {
            buffer: source_buffer,
            pro_que: ocl_pq,
            num_amps,
            num_qubits: bits.len() as u32,
            backend,

            #[cfg(feature = "decoherence")]
            decoherence: 0.0,
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

        #[cfg(feature = "decoherence")]
        self.decohere();
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

        #[cfg(feature = "decoherence")]
        self.decohere();
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
    /// as a HashMap, with the key as the result and the value as the
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

    /// This method allows you to simulate the effects of decoherence on
    /// the simulated quantum computer. The decoherence is simulated through phase dapening.
    /// The argument `d` will be used as the strength factor
    ///
    /// Note that setting the decoherence will make the simulator have to preform twice the
    /// number of calculations, as the `decohere` function is called after each state changing
    /// method.
    #[cfg(feature = "decoherence")]
    pub fn set_decoherence(&mut self, d: f32) {
        self.decoherence = d;
    }

    /// Preforms the actual decoherence of a quantum register based on the parameter `self.decoherence`
    #[inline]
    #[cfg(feature = "decoherence")]
    pub fn decohere(&mut self) {
        if self.decoherence != 0.0 {
            /*
            /**
 * Renders decoherence.
 * @param {number} strength The strength of the decoherence.
 */
quantum.Simulator.prototype.applyDecoherence = function(strength) {
  /* Generate normal distributed random numbers */
  for (var i = 0; i < this.vectorSize; i++) {
    do {
      var u = 2 * Math.random() - 1;
      var v = 2 * Math.random() - 1;
      var s = u * u + v * v;
    } while (s >= 1);
    var x = u * Math.sqrt(-2 * Math.log(s) / s);
    x *= Math.sqrt(2 * strength);
    this.decoherenceShader.uniforms.nrands.value[i] = x / 2;
  }
  for (var i = this.vectorSize; i < 22; i++) {
    this.decoherenceShader.uniforms.nrands.value[i] = 0;
  }
  this.renderShader(this.decoherenceShader);
};*/
        }
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

    /// Preform multiple measurements of the first `num_to_measure`, returning the results
    /// as a HashMap, with the key as the result and the value as the
    /// number of times that result was measured
    pub fn measure_first(
        &mut self,
        num_to_measure: i32,
        num_iterations: i32,
    ) -> HashMap<String, i32> {
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
            let num_chars = state.len();
            let result = state
                .chars()
                .skip(num_chars - num_to_measure as usize)
                .take(num_to_measure as usize)
                .collect();
            let count = num_results.entry(result).or_insert(0);
            *count += 1;
        }

        num_results
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

        #[cfg(feature = "decoherence")]
        self.decohere();
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

        #[cfg(feature = "decoherence")]
        self.decohere();
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
