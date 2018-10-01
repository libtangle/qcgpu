import os
import random
import numpy as np
import pyopencl as cl
import pyopencl.array as pycl_array
from pyopencl.reduction import ReductionKernel

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

        self.program.apply_gate(
            self.queue,
            [int(self.buffer.shape[0] / 2)],
            None,
            self.buffer.data,
            np.int32(target),
            self.dtype(gate.a),
            self.dtype(gate.b),
            self.dtype(gate.c),
            self.dtype(gate.d)
        )

    def apply_controlled_gate(self, gate, control, target):
        """Applies a controlled gate to the quantum register"""

        self.program.apply_controlled_gate(
            self.queue,
            [int(self.buffer.shape[0] / 2)],
            None,
            self.buffer.data,
            np.int32(control),
            np.int32(target),
            self.dtype(gate.a),
            self.dtype(gate.b),
            self.dtype(gate.c),
            self.dtype(gate.d)
        )
    
    def qubit_probability(self, target):
        """Get the probability of a single qubit begin measured as '0'"""

        preamble = """
        #include <pyopencl-complex.h>

        float probability(int target, int i, cfloat_t amp) {
            if ((i & (1 << target )) != 0) {
                return 0;
            }
            // return 6.0;
            float abs = cfloat_abs(amp);
            return abs * abs;
        }
        """
        

        kernel = ReductionKernel(
            self.context, 
            np.float, 
            neutral = "0",
            reduce_expr="a + b",
            map_expr="probability(target, i, amps[i])",
            arguments="__global cfloat_t *amps, __global int target",
            preamble=preamble
        )

        return kernel(self.buffer, target).get()

    def measure_qubit(self, target, samples):
        probability_of_0 = self.qubit_probability(target)

        total = 0

        for i in range(samples):
            outcome = 1 if random.random() > probability_of_0 else 0
            total = total + outcome
            
        
        return total


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
        
    def release(self):
        self.buffer.base_data.release()
