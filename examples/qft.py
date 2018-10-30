"""
QFT
=====================

This is an implementation of the quantum Fourier transform.
"""

import qcgpu
import math

def qft():
    print('start')
    state = qcgpu.State(24)
    num_qubits = state.num_qubits

    for j in range(num_qubits):
        for k in range(j):
            state.cu1(j, k, math.pi/float(2**(j-k)))
        state.h(j)

if __name__== "__main__":
    qft()