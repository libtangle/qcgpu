import os
import random
import numpy as np
import pyopencl as cl
import pyopencl.array as pycl_array
from pyopencl.reduction import ReductionKernel
from collections import defaultdict

# Get the OpenCL kernel
kernel = """
#include <pyopencl-complex.h>

/*
 * Returns the nth number where a given digit
 * is cleared in the binary representation of the number
 */
static int nth_cleared(int n, int target)
{
    int mask = (1 << target) - 1;
    int not_mask = ~mask;

    return (n & mask) | ((n & not_mask) << 1);
}

///////////////////////////////////////////////
// KERNELS
///////////////////////////////////////////////

/*
 * Applies a single qubit gate to the register.
 * The gate matrix must be given in the form:
 *
 *  A B
 *  C D
 */
__kernel void apply_gate(
    __global cfloat_t *amplitudes,
    int target,
    cfloat_t A,
    cfloat_t B,
    cfloat_t C,
    cfloat_t D)
{
    int const global_id = get_global_id(0);

    int const zero_state = nth_cleared(global_id, target);

    // int const zero_state = state & (~(1 << target)); // Could just be state
    int const one_state = zero_state | (1 << target);

    cfloat_t const zero_amp = amplitudes[zero_state];
    cfloat_t const one_amp = amplitudes[one_state];

    amplitudes[zero_state] = cfloat_add(cfloat_mul(A, zero_amp), cfloat_mul(B, one_amp));
    amplitudes[one_state] = cfloat_add(cfloat_mul(D, one_amp), cfloat_mul(C, zero_amp));
}

/*
 * Applies a controlled single qubit gate to the register.
 */
__kernel void apply_controlled_gate(
    __global cfloat_t *amplitudes,
    int control,
    int target,
    cfloat_t A,
    cfloat_t B,
    cfloat_t C,
    cfloat_t D)
{
    int const global_id = get_global_id(0);
    int const zero_state = nth_cleared(global_id, target);
    int const one_state = zero_state | (1 << target); // Set the target bit

    int const control_val_zero = (((1 << control) & zero_state) > 0) ? 1 : 0;
    int const control_val_one = (((1 << control) & one_state) > 0) ? 1 : 0;

    cfloat_t const zero_amp = amplitudes[zero_state];
    cfloat_t const one_amp = amplitudes[one_state];

    if (control_val_zero == 1)
    {
        amplitudes[zero_state] = cfloat_add(cfloat_mul(A, zero_amp), cfloat_mul(B, one_amp));
    }

    if (control_val_one == 1)
    {
        amplitudes[one_state] = cfloat_add(cfloat_mul(D, one_amp), cfloat_mul(C, zero_amp));
    }
}

/*
 * Applies a controlled-controlled single qubit gate to the register.
 */
__kernel void apply_controlled_controlled_gate(
    __global cfloat_t *amplitudes,
    int control,
    int control_2,
    int target,
    cfloat_t A,
    cfloat_t B,
    cfloat_t C,
    cfloat_t D)
{
    int const global_id = get_global_id(0);
    int const zero_state = nth_cleared(global_id, target);
    int const one_state = zero_state | (1 << target); // Set the target bit

    int const control_val_zero = (((1 << control) & zero_state) > 0) ? 1 : 0;
    int const control_val_one = (((1 << control) & one_state) > 0) ? 1 : 0;
    int const control_val_two_zero = (((1 << control_2) & zero_state) > 0) ? 1 : 0;
    int const control_val_two_one = (((1 << control_2) & one_state) > 0) ? 1 : 0;

    cfloat_t const zero_amp = amplitudes[zero_state];
    cfloat_t const one_amp = amplitudes[one_state];

    if (control_val_zero == 1 && control_val_two_zero == 1)
    {
        amplitudes[zero_state] = cfloat_add(cfloat_mul(A, zero_amp), cfloat_mul(B, one_amp));
    }

    if (control_val_one == 1 && control_val_two_one == 1)
    {
        amplitudes[one_state] = cfloat_add(cfloat_mul(D, one_amp), cfloat_mul(C, zero_amp));
    }
}

/*
 * Swaps the states of two qubits in the register
 * NOT MIGRATED
 */
// __kernel void swap(
//     __global cfloat_t *const amplitudes,
//     __global cfloat_t *amps,
//     int first_qubit,
//     int second_qubit)
// {
//     int const state = get_global_id(0);

//     int const first_bit_mask = 1 << first_qubit;
//     int const second_bit_mask = 1 << second_qubit;

//     int const new_second_bit = ((state & first_bit_mask) >> first_qubit) << second_qubit;
//     int const new_first_bit = ((state & second_bit_mask) >> second_qubit) << first_qubit;

//     int const new_state = (state & !first_bit_mask & !second_bit_mask) | new_first_bit | new_second_bit;

//     amps[new_state] = amplitudes[state];
// }


/**
 * Get a single amplitude
 */
__kernel void get_single_amplitude(
    __global cfloat_t *const amplitudes,
    __global cfloat_t *out,
    int i)
{
    out[0] = amplitudes[i];
}

/**
 * Calculates The Probabilities Of A State Vector
 */
__kernel void calculate_probabilities(
    __global cfloat_t *const amplitudes,
    __global float *probabilities)
{
    int const state = get_global_id(0);
    cfloat_t amp = amplitudes[state];

    probabilities[state] = cfloat_abs(cfloat_mul(amp, amp));
}

/**
 * Initializes a register to the value 1|0..100...0>
 *                                          ^ target
 */
__kernel void initialize_register(
    __global cfloat_t *amplitudes,
    int const target)
{
    int const state = get_global_id(0);
    if (state == target)
    {
        amplitudes[state] = cfloat_new(1, 0);
    }
    else
    {
        amplitudes[state] = cfloat_new(0, 0);
    }
}

/**
 * Collapses a qubit in the register
 */
__kernel void collapse(
    __global cfloat_t *amplitudes, 
    int const target,
    int const outcome, 
    float const norm)
{
    int const state = get_global_id(0);

    if (((state >> target) & 1) == outcome) {
        amplitudes[state] = cfloat_mul(amplitudes[state], cfloat_new(norm, 0.0));
    }
    else
    {
        amplitudes[state] = cfloat_new(0.0, 0.0);
    }
}
"""

# Setup the OpenCL Context here to not prompt every execution
context = None
program = None


class Backend:
    """
    A class for the OpenCL backend to the simulator.

    This class shouldn't be used directly, as many of the
    methods don't have the same input checking as the State
    class.
    """

    # @profile
    def __init__(self, num_qubits, dtype=np.complex64):
        if not context:
            create_context()
        
        """
        Initialize a new OpenCL Backend

        Takes an argument of the number of qubits to use
        in the register, and returns the backend.
        """
        self.num_qubits = num_qubits
        self.dtype = dtype

        self.queue = cl.CommandQueue(context)

        # Buffer for the state vector
        self.buffer = pycl_array.to_device(
            self.queue,
            np.eye(1, 2**num_qubits, dtype=dtype)
        )

    def apply_gate(self, gate, target):
        """Applies a gate to the quantum register"""
        program.apply_gate(
            self.queue,
            [int(2**self.num_qubits / 2)],
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

        program.apply_controlled_gate(
            self.queue,
            [int(2**self.num_qubits / 2)],
            None,
            self.buffer.data,
            np.int32(control),
            np.int32(target),
            self.dtype(gate.a),
            self.dtype(gate.b),
            self.dtype(gate.c),
            self.dtype(gate.d)
        )
    
    def apply_controlled_controlled_gate(self, gate, control1, control2, target):
        """Applies a controlled controlled gate (such as a toffoli gate) to the quantum register"""

        program.apply_controlled_controlled_gate(
            self.queue,
            [int(2**self.num_qubits / 2)],
            None,
            self.buffer.data,
            np.int32(control1),
            np.int32(control2),
            np.int32(target),
            self.dtype(gate.a),
            self.dtype(gate.b),
            self.dtype(gate.c),
            self.dtype(gate.d)
        )

    def seed(self, val):
        random.seed(val)
        
    def measure(self, samples=1):
        """Measure the state of a register"""
        # This is a really horrible method that needs a rewrite - the memory
        # is attrocious

        probabilities = self.probabilities()
        # print(probabilities)
        # print(np.sum(self.amplitudes()))
        choices = np.random.choice(
            np.arange(0, 2**self.num_qubits), 
            samples, 
            p=probabilities
        )
        
        results = defaultdict(int)
        for i in choices:
            results[np.binary_repr(i, width=self.num_qubits)] += 1
        
        return dict(results)

    def measure_first(self, num, samples):
        probabilities = self.probabilities()
        # print(probabilities)
        # print(np.sum(self.amplitudes()))
        choices = np.random.choice(
            np.arange(0, 2**self.num_qubits), 
            samples, 
            p=probabilities
        )
        
        results = defaultdict(int)
        for i in choices:
            key = np.binary_repr(i, width=self.num_qubits)[-num:]
            results[key] += 1
        
        return dict(results)
       

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
            context, 
            np.float, 
            neutral = "0",
            reduce_expr="a + b",
            map_expr="probability(target, i, amps[i])",
            arguments="__global cfloat_t *amps, __global int target",
            preamble=preamble
        )

        return kernel(self.buffer, target).get()

    def measure_collapse(self, target):
        probability_of_0 = self.qubit_probability(target)
        random_number = random.random()

        if random_number <= probability_of_0:
            outcome = '0'
            norm = 1 / np.sqrt(probability_of_0)
        else:
            outcome = '1'
            norm = 1 / np.sqrt(1 - probability_of_0)

        program.collapse(
            self.queue,
            [int(2**self.num_qubits)],
            # 2**self.num_qubits,
            None,
            self.buffer.data,
            np.int32(target),
            np.int32(outcome),
            np.float32(norm)
        )
        return outcome

    def measure_qubit(self, target, samples):
        probability_of_0 = self.qubit_probability(target)

        choices = np.random.choice(
            [0, 1], 
            samples, 
            p=[probability_of_0, 1-probability_of_0]
        )
        
        results = defaultdict(int)
        for i in choices:
            results[np.binary_repr(i, width=1)] += 1
        
        return dict(results)

    def single_amplitude(self, i):
        """Gets a single probability amplitude"""
        out = pycl_array.to_device(
            self.queue,
            np.empty(1, dtype=np.complex64)
        )

        program.get_single_amplitude(
            self.queue, 
            (1, ), 
            None, 
            self.buffer.data,
            out.data,
            np.int32(i)
        )

        return out[0]

    def amplitudes(self):
        """Gets the probability amplitudes"""
        return self.buffer.get()
    
    def probabilities(self):
        """Gets the squared absolute value of each of the amplitudes"""
        out = pycl_array.to_device(
            self.queue,
            np.zeros(2**self.num_qubits, dtype=np.float32)
        )

        program.calculate_probabilities(
            self.queue,
            out.shape,
            None,
            self.buffer.data,
            out.data
        )

        return out.get()
        
    def release(self):
        self.buffer.base_data.release()
    
def create_context():
    global context
    global program
    context = cl.create_some_context()
    program = cl.Program(context, kernel).build(options="-cl-no-signed-zeros -cl-mad-enable -cl-fast-relaxed-math")
