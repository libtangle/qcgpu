from projectq import MainEngine
import projectq.ops as ops
from projectq.backends import Simulator
import sys
import time

if len(sys.argv) > 1:
    n = int(sys.argv[1])
else:
    n = 16

if len(sys.argv) > 1:
    depth = int(sys.argv[2])
else:
    depth = 10

print('Qubits: %d, Depth %d' % (n, depth))


eng = MainEngine(backend=Simulator(gate_fusion=True), engine_list=[])
qbits = eng.allocate_qureg(n)

start = time.time()

for level in range(depth):
    for q in qbits:
        ops.H | q
        ops.SqrtX | q
        if q != qbits[0]:
            ops.CNOT | (q, qbits[0])



runtime = time.time() - start

for q in qbits:
    ops.Measure | q
    
print(runtime)

