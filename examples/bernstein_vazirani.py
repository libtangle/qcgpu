# -*- coding: utf-8 -*-

"""
Bernstein-Vazirani Algorithm
============================

This algorithm finds a hidden integer :math:`a \in \{ 0, 1\}^n` from
an oracle :math:`f_a` which returns a bit :math:`a \cdot x \equiv \sum_i a_i x_i \mod 2`
for an input :math:`x \in \{0,1\}^n`.

A classical oracle returns :math:`f_a(x) = a \dot x \mod 2`, while the quantum oracle
must be queried with superpositions of input :math:`x`'s.

To solve this problem classically, the hidden integer can be found by checking the
oracle with the inputs :math:`x = 1,2,\dots,2^i,2^{n-1}`, where each
query reveals the :math:`i`th bit of :math:`a` (:math:`a_i`).
This is the optimal classical solution, and is :math:`O(n)`. Using a quantum oracle and the
Bernstein-Vazirani algorithm, :math:`a` can be found with just one query to the oracle.

The Algorithm
-------------

1. Initialize :math:`n` qubits in the state :math:`\lvert 0, \dots, 0\rangle`.
2. Apply the Hadamard gate :math:`H` to each qubit.
3. Apply the inner product oracle.
4. Apply the Hadamard gate :math:`H` to each qubit.
5. Measure the register

From this procedure, we find that the registers measured value is equal to that of
the original hidden integer.
"""

def bernstein_vazirani():
    import qcgpu

    a = 101 # The hidden integer, bitstring is 1100101

    register = qcgpu.State(16) # Create a new quantum register

    register.apply_all(qcgpu.gate.h()) # Apply a hadamard gate to each qubit

    # Apply the inner products oracle
    for i in range(16):
        if a & (1 << i) != 0:
            register.z(i)

    register.apply_all(qcgpu.gate.h()) # Apply a hadamard gate to each qubit

    register.measure(samples=1000) # Measure the register (sample 1000 times)

if __name__== "__main__":
  bernstein_vazirani()