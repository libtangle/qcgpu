use ocl::{Buffer, MemFlags, ProQue};
use ocl::enums::DeviceInfo::Type;
use kernel::KERNEL;
use gates::Gate;

#[derive(Debug)]
pub struct State {
    pub buffer: Buffer<f32>,
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

        let mut source = vec![0.0f32; ocl_pq.dims().to_len()];
        source[0] = 1f32;
        // let source = vec![1.0f32, 0.0, 0.0, 0.0];

        // create a temporary vector with the source buffer
        let source_buffer = Buffer::builder()
            .queue(ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write().copy_host_ptr())
            .len(num_amps)
            .host_data(&source)
            .build()
            .expect("Source Buffer");

        State {
            buffer: source_buffer,
            pro_que: ocl_pq,
            num_amps: num_amps,
            num_qubits: num_qubits,
        }
    }

    pub fn apply_gate(&mut self, target: i32, gate: Gate) {
        let source = vec![0.0f32; self.pro_que.dims().to_len()];

        // create a temporary vector with the source buffer
        let result_buffer = Buffer::builder()
            .queue(self.pro_que.queue().clone())
            .flags(MemFlags::new().read_write().copy_host_ptr())
            .len(self.num_amps)
            .host_data(&source)
            .build()
            .expect("Result Buffer");

        let apply = self.pro_que
            .create_kernel("apply_gate")
            .unwrap()
            .arg_buf(&self.buffer)
            .arg_buf(&result_buffer)
            .arg_scl(target)
            .arg_scl(gate.a)
            .arg_scl(gate.b)
            .arg_scl(gate.c)
            .arg_scl(gate.d);

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

    pub fn print(&self) {
        let mut vec_result = vec![0.0f32; self.num_amps];
        // Read results from the device into result_buffer's local vector:
        &self.buffer.read(&mut vec_result).enq().unwrap();

        for idx in 0..self.num_amps {
            print!("[{idx}]: {}, ", vec_result[idx], idx = idx);
        }
        print!("\n");
    }

    pub fn info(&self) {
        println!("{:?}", self.pro_que.device().info(Type).unwrap())
    }
}
