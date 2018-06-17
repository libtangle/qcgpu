use traits::Backend;
use gate::Gate;

use ocl::{Buffer, MemFlags, ProQue};
use num_complex::{Complex, Complex32};
use rand::random;
use std::fmt;
use failure::Error;

// OpenCL Kernel
pub static KERNEL: &'static str = include_str!("kernel.cl");

#[derive(Debug)]
pub struct OpenCL {
    /// OpenCL Buffer for the state vector
    pub buffer: Buffer<Complex<f32>>,
    pro_que: ProQue,
    num_qubits: u8,
}

impl OpenCL {
    pub fn new(num_qubits: u8) -> Result<OpenCL, Error> {
        // How many amplitudes needed?
        let num_amps = 2_usize.pow(u32::from(num_qubits)) as usize;

        let ocl_pq = ProQue::builder()
            .src(KERNEL)
            .device(1)
            .dims(num_amps)
            .build()?;

        let buffer: Buffer<Complex32> = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write())
            .len(num_amps)
            .build()?;

        let apply = ocl_pq
            .kernel_builder("initialize_register")
            .arg(&buffer)
            .arg(0)
            .build()?;

        unsafe {
            apply.enq()?;
        }

        Ok(OpenCL {
            pro_que: ocl_pq,
            buffer,
            num_qubits,
        })
    }

    fn get_probabilities(&self) -> Result<Vec<f32>, Error> {
        let result_buffer: Buffer<f32> = self.pro_que.create_buffer()?;

        let apply = self.pro_que
            .kernel_builder("calculate_probabilities")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .build()?;

        unsafe {
            apply.enq()?;
        }

        let mut vec_result = vec![0.0f32; self.buffer.len()];
        result_buffer.read(&mut vec_result).enq()?;

        Ok(vec_result)
    }
}

impl Backend for OpenCL {
    fn apply_gate(&mut self, gate: Gate, target: u8) -> Result<(), Error> {
        // create a temporary vector with the source buffer
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer()?;

        let apply = self.pro_que
            .kernel_builder("apply_gate")
            .arg(&self.buffer)
            .arg(&result_buffer)
            .arg(i32::from(target))
            .arg(gate.a)
            .arg(gate.b)
            .arg(gate.c)
            .arg(gate.d)
            .build()?;

        unsafe {
            apply.enq()?;
        }

        self.buffer = result_buffer;
        Ok(())
    }

    fn apply_controlled_gate(&mut self, gate: Gate, control: u8, target: u8) -> Result<(), Error> {
        let result_buffer: Buffer<Complex32> = self.pro_que.create_buffer()?;

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
            .build()?;

        unsafe {
            apply.enq()?;
        }

        self.buffer = result_buffer;

        Ok(())
    }

    fn measure(&mut self) -> Result<u8, Error> {
        let probabilities = self.get_probabilities()?;

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

        Ok(i as u8)
    }

    fn measure_qubit(&mut self, target: u8) -> Result<u8, Error> {
        unimplemented!()
    }

    fn num_qubits(&self) -> u8 {
        self.num_qubits
    }
}

impl fmt::Display for OpenCL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        let mut vec_result = vec![Complex32::new(0.0, 0.0); self.buffer.len()];
        self.buffer
            .read(&mut vec_result)
            .enq()
            .expect("Error Reading Memory From Device");

        for (idx, item) in vec_result.iter().enumerate() {
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }

            write!(f, "[{}]: ", idx)?;

            // Do we print the imaginary part?
            if item.im == 0.0 {
                write!(f, "{}", item.re)?;
            } else if item.re == 0.0 {
                write!(f, "{}i", item.im)?;
            } else {
                write!(f, "{}", item)?;
            }
        }

        Ok(())
    }
}
