from __future__ import print_function
import qcgpu

s = qcgpu.State(3)
h = qcgpu.gate.h()

print(h)

s.apply_gate(h, 0)

print(s)

# from __future__ import absolute_import
# from __future__ import print_function
# import pyopencl as cl
# import pyopencl.array as cl_array
# import numpy
# import numpy.linalg as la

# a = numpy.random.rand(8).astype(numpy.complex64)

# ctx = cl.create_some_context()
# queue = cl.CommandQueue(ctx)

# a_dev = cl_array.to_device(queue, a)

# prg = cl.Program(ctx, """
#     #include <pyopencl-complex.h>

#     /*
#     * Returns the nth number where a given digit
#     * is cleared in the binary representation of the number
#     */
#     static int nth_cleared(int n, int target)
#     {
#         int mask = (1 << target) - 1;
#         int not_mask = ~mask;

#         return (n & mask) | ((n & not_mask) << 1);
#     }

#     /*
#     * Applies a single qubit gate to the register.
#     * The gate matrix must be given in the form:
#     *
#     *  A B
#     *  C D
#     */
#     __kernel void apply_gate(
#         __global cfloat_t *amplitudes,
#         int target,
#         cfloat_t A,
#         cfloat_t B,
#         cfloat_t C,
#         cfloat_t D)
#     {
#         int const global_id = get_global_id(0);

#         int const zero_state = nth_cleared(global_id, target);

#         // int const zero_state = state & (~(1 << target)); // Could just be state
#         int const one_state = zero_state | (1 << target);

#         cfloat_t const zero_amp = amplitudes[zero_state];
#         cfloat_t const one_amp = amplitudes[one_state];

#         amplitudes[zero_state] = cfloat_add(cfloat_mul(A, zero_amp), cfloat_mul(B, one_amp));
#         amplitudes[one_state] = cfloat_add(cfloat_mul(D, one_amp), cfloat_mul(C, zero_amp));
#     }

#     __kernel void sum(__global cfloat_t *a, cfloat_t b)
#     {
#       int gid = get_global_id(0);
#       a[gid] = cfloat_add(a[gid], b);
#     }
#     """).build()

# # kernel = prg.sum
# # kernel.set_scalar_arg_dtypes([
# #     None,
# #     None,
# #     numpy.complex64
# # ])

# prg.sum(queue, (int(a.shape[0] / 2),), None, a_dev.data, numpy.complex64(3+2j))

# prg.sum(queue, (2, ), None, a_dev.data, numpy.complex64(3+2j))

# print(a_dev)

# print((a.shape[0] / 2,))