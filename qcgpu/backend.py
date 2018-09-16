import os
import pyopencl as cl
import pyopencl.array as pycl_array
import numpy as np

# Get the OpenCL kernel
kernel_path = os.path.join(
    os.path.dirname(__file__),
    "kernels/brute-force.cl"
)
kernel = open(kernel_path, "r").read()


class Backend:
    """
    A class for the OpenCL backend to the simulator.

    This class shouldn't be used directly, as many of the
    methods don't have the same input checking as the State
    class.
    """

    def __init__(self, num_qubits, dtype=np.complex64):
        """
        Initialize a new OpenCL Backend

        Takes an argument of the number of qubits to use
        in the register, and returns the backend.
        """
        self.num_qubits = num_qubits
        self.dtype = dtype

        # Setup the opencl context, queue and program
        self.context = cl.create_some_context()
        self.queue = cl.CommandQueue(self.context)
        self.program = cl.Program(self.context, kernel).build()

        # Buffer for the state vector
        self.buffer = pycl_array.to_device(
            self.queue,
            np.eye(2**num_qubits, 1, dtype=dtype)
        )

    def apply_gate(self, gate, target):
        """Applies a gate to the quantum register"""

        kernel = self.program.apply_gate
        kernel.set_scalar_arg_dtypes([
            None,
            np.int32,
            self.dtype,
            self.dtype,
            self.dtype,
            self.dtype,
        ])
        kernel(
            self.queue,
            [self.buffer.shape[0] / 2],
            None,
            self.buffer.data,
            np.int32(target),
            gate.a,
            gate.b,
            gate.c,
            gate.d
        )

    def apply_controlled_gate(self, gate, control, target):
        """Applies a controlled gate to the quantum register"""

        kernel = self.program.apply_controlled_gate
        kernel.set_scalar_arg_dtypes([
            None,
            np.int32,
            np.int32,
            self.dtype,
            self.dtype,
            self.dtype,
            self.dtype,
        ])
        kernel(
            self.queue,
            [self.buffer.shape[0] / 2],
            None,
            self.buffer.data,
            np.int32(control),
            np.int32(target),
            gate.a,
            gate.b,
            gate.c,
            gate.d
        )
    
    def amplitudes(self):
        """Gets the probability amplitudes"""
        return self.buffer.get()
    
    def probabilities(self):
        """Gets the squared absolute value of each of the amplitudes"""
        out = pycl_array.to_device(
            self.queue,
            np.zeros(2**self.num_qubits, dtype=np.float32)
        )

        self.program.calculate_probabilities(
            self.queue,
            self.buffer.shape,
            None,
            self.buffer.data,
            out.data
        )

        return out.get()
        
