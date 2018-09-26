# import qcgpu
# import perf

# def run(num_qubits, depth):
#     state = qcgpu.State(num_qubits)
#     h = qcgpu.gate.h()

#     for i in range(num_qubits * depth):
#         state.apply_gate(h, i % num_qubits)

import qcgpu
import sys
import time

# ------------------------------------------
# number of qubits and depth
# ------------------------------------------
if len(sys.argv) > 1:
    n = int(sys.argv[1])
else:
    n = 16

if len(sys.argv) > 1:
    depth = int(sys.argv[2])
else:
    depth = 10

print('Qubits: %d, Depth %d' % (n, depth))


# ------------------------------------------
# qubit register
# ------------------------------------------

state = qcgpu.State(n)

# ------------------------------------------
# circuit
# ------------------------------------------

h = qcgpu.gate.h()
x = qcgpu.gate.x()
sqrt_x = qcgpu.gate.sqrt_x()

# timing -- get the start time
start = time.time()

# random circuit
for level in range(depth):
    for q in range(n):
        state.apply_gate(h, q)
        state.apply_gate(sqrt_x, q)

        if q != 0:
            state.apply_controlled_gate(x, q, 0)

# timing -- get the end time
runtime = time.time() - start

# print out the runtime
print(runtime)