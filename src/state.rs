use ocl::{Buffer, MemFlags, ProQue};
use ocl::enums::DeviceInfo::Type;
use num_complex::Complex32;

use kernel::KERNEL;
use gates::Gate;

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

    pub fn print(&self) {
        let mut vec_result = vec![Complex32::new(0.0, 0.0); self.num_amps];
        // Read results from the device into result_buffer's local vector:
        self.buffer.read(&mut vec_result).enq().unwrap();

        for (idx, item) in vec_result.iter().enumerate() {
            print!("[{idx}]: {}, ", item, idx = idx);
        }
        println!();
    }

    pub fn info(&self) {
        println!("{:?}", self.pro_que.device().info(Type).unwrap())
    }
}
