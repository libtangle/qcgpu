# -*- coding: utf-8 -*-

"""
Bell State / EPR Pair
=====================

The Bell State, also known as the EPR pair (after Einstein, Podosky and Rosen)
is the simplest example of entanglement.

The Bell State is defined as the maximally entangled quantum state of two qubits.
"""

def bell_state():
    import qcgpu

    print("Creating Bell State")

    state = qcgpu.state(2)

    state.h(0)
    state.cx(0, 1)

    print("Measurement Results:")
    print(state.measure(samples = 1000))

if __name__== "__main__":
  bell_state()