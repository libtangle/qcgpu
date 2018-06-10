use traits::Backend;
use gate::Gate;

use ocl::{Buffer, MemFlags, ProQue};
use num_complex::{Complex, Complex32};
use rand::random;
use std::fmt;

// OpenCL Kernel
pub static KERNEL: &'static str = include_str!("kernel.cl");

#[derive(Debug)]
pub struct OpenCL {
    /// OpenCL Buffer for the state vector
    pub buffer: Buffer<Complex<f32>>,
    pro_que: ProQue,
}

impl OpenCL {
    pub fn new(num_qubits: u8) -> OpenCL {
        // How many amplitudes needed?
        let num_amps = 2_usize.pow(u32::from(num_qubits)) as usize;

        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(0)
            .dims(num_amps)
            .build()
            .expect("Error Building ProQue");

        let buffer: Buffer<Complex32> = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write())
            .len(num_amps)
            .build()
            .expect("Source Buffer");

        let apply = ocl_pq
            .kernel_builder("initialize_register")
            .arg(&buffer)
            .arg(0)
            .build()
            .unwrap();

        unsafe {
            apply.enq().unwrap();
        }

        OpenCL {
            pro_que: ocl_pq,
            buffer,
        }
    }

    fn get_probabilities(&self) -> Vec<f32> {
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

        let mut vec_result = vec![0.0f32; self.buffer.len()];
        result_buffer.read(&mut vec_result).enq().unwrap();

        vec_result
    }
}

impl Backend for OpenCL {
    fn apply_gate(&mut self, gate: Gate, target: u8) {
        // create a temporary vector with the source buffer
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer().unwrap();

        let apply = self.pro_que
            .kernel_builder("apply_gate")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .arg(i32::from(target))
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

    fn apply_controlled_gate(&mut self, gate: Gate, control: u8, target: u8) {
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

    fn measure(&self) -> u8 {
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

        i as u8
    }
}

impl fmt::Display for OpenCL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        let mut vec_result = vec![Complex32::new(0.0, 0.0); self.buffer.len()];
        self.buffer.read(&mut vec_result).enq().unwrap();

        for (idx, item) in vec_result.iter().enumerate() {
            if !first {
                write!(f, ", ").unwrap();
            } else {
                first = false;
            }

            write!(f, "[{}]: ", idx).unwrap();

            // Do we print the imaginary part?
            if item.im == 0.0 {
                write!(f, "{}", item.re).unwrap();
            } else if item.re == 0.0 {
                write!(f, "{}i", item.im).unwrap();
            } else {
                write!(f, "{}", item).unwrap();
            }
        }

        Ok(())
    }
}