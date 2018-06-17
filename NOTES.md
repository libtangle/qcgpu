# Notes

> This document is to keep track of various ideas, plans ect. relating
> to the development of QCGPU.

## Simulators

At a bare minimum, a quantum computer simulator must have the following parts:

* Ability to create a quantum register with a given numebr of qubits
* Ability to apply controlled gates, or at minimum the controlled pauli-x gate (CNOT)
* Ability to apply single qubit gates, which combined with the CNOT form a universal set of gates.
* Ability to measure single qubits in the register and collapse their state into the measured state.

Other functionality can be added which will make the simulator more useful, which is discussed later
