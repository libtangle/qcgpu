# Reporting Imports
import time
import random
import statistics

# QCGPU Implementation
import qcgpu

def bench_qcgpu(n, depth):
    h = qcgpu.gate.h()
    x = qcgpu.gate.x()
    t = qcgpu.gate.t()

    state = qcgpu.State(n)

    start = time.time()

    for level in range(depth):
        for q in range(n):
    
            state.apply_gate(h, q)
            state.apply_gate(t, q)

            if q != 0:
                state.apply_controlled_gate(x, q, 0)
    return time.time() - start

# Qiskit Implementation
from qiskit import ClassicalRegister, QuantumRegister
from qiskit import QuantumCircuit, execute

def bench_qiskit(n, depth):
    q = QuantumRegister(n)
    c = ClassicalRegister(n)
    qc = QuantumCircuit(q, c)
    
    for level in range(depth):
        for i in range(n):
            qc.h(q[i])
            qc.t(q[i])

            if i != 0:
                qc.cx(q[i], q[0])
    qc.measure(q, c)

    start = time.time()
    
    job_sim = execute(qc, "local_statevector_simulator")
    
    return time.time() - start

# ProjectQ Implementation
from projectq import MainEngine
import projectq.ops as ops
from projectq.backends import Simulator
from projectq.types import Qureg

def bench_projectq(n, depth):
    eng = MainEngine(backend=Simulator(gate_fusion=True), engine_list=[])
    qbits = eng.allocate_qureg(n)

    start = time.time()

    for level in range(depth):
        for q in qbits:
            ops.H | q
            ops.T | q
            if q != qbits[0]:
                ops.CNOT | (q, qbits[0])

    runtime = time.time() - start

    for q in qbits:
        ops.Measure | q
    eng.flush()
    return runtime

# Reporting

functions = bench_qcgpu, bench_qiskit, bench_projectq

times = {f.__name__: [] for f in functions}

names = []
means = []

samples = 10
for i in range(samples):  # adjust accordingly so whole thing takes a few sec
    progress = i / samples
    print("\rProgress: [{0:50s}] {1:.1f}%".format('#' * int(progress * 50), progress*100), end="", flush=True)
    func = random.choice(functions)
    t = func(5,1)
    times[func.__name__].append(t)

print('')

for name, numbers in times.items():
    print('FUNCTION:', name, 'Used', len(numbers), 'times')
    print('\tMEDIAN', statistics.median(numbers))
    print('\tMEAN  ', statistics.mean(numbers))
    print('\tSTDEV ', statistics.stdev(numbers))
    means.append(statistics.mean(numbers))
    names.append(name)

# Graphing

import numpy as np
import matplotlib.pyplot as plt

index = np.arange(len(names))
print(index)

plt.bar(index, means)
plt.xlabel('Function')
plt.ylabel('Time (s)')
plt.xticks(index, names)
plt.title('Performance')
plt.show()