///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////use ocl::{Buffer, MemFlags, ProQue};
use ocl::enums::DeviceInfo::Type;
use ocl::{Buffer, ProQue, MemFlags};
use num_complex::Complex32;
use std::fmt;
use rand::random;

use kernel::KERNEL;
use gates::Gate;

/// Representation of a quantum register

#[derive(Debug)]
pub struct State {
    pub buffer: Buffer<Complex32>,
    pub pro_que: ProQue,
    pub num_amps: usize,
    pub num_qubits: u32,
}

impl State {
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
        }
    }

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

    pub fn apply_all(&mut self, gate: Gate) {
        for i in 0..self.num_qubits as i32 {
            self.apply_gate(i, gate);
        }
    }

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

    pub fn measure(&mut self) -> i32 {
        let probabilities = self.get_probabilities();

        let mut key = random::<f32>();
        if key > 1.0 {
            key = key % 1.0;
        }

        let mut i = 0;
        while i < probabilities.len() {
            key = key - probabilities[i];
            if key <= 0.0 {
                break;
            }
            i = i + 1;
        }

        i as i32
    }

    pub fn info(&self) {
        println!("{:?}", self.pro_que.device().info(Type).unwrap())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        let mut vec_result = vec![Complex32::new(0.0, 0.0); self.num_amps];
        self.buffer.read(&mut vec_result).enq().unwrap();

        for (idx, item) in vec_result.iter().enumerate() {
            if first {
                write!(f, "[{idx}]: {}", item, idx = idx);
                first = false;
            } else {
                write!(f, ", [{idx}]: {}", item, idx = idx);
            }
        }

        Ok(())
    }
}
