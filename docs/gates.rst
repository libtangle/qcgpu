Quantum Gates
=============

In quantum computing, gates are used to manipulate quantum registers and
to implement quantum algorithms.

Built-In Gates
--------------

There are a number of gates built into QCGPU. They can all be applied
the same way:

.. code:: python

   import qcgpu

   state = qcgpu.State(2)

   state.h(0) # Applies the Hadamard (H) gate to the first qubit.
   state.x(1) # Applies a pauli-x (NOT) gate to the second qubit.

``h`` and ``x`` can be replaced with any of the following:

-  The Hadamard gate: **h** - ``state.h(0)``
-  The S gate: **s** - ``state.s(0)``
-  The T gate: **t** - ``state.t(0)``
-  The Pauli-X / NOT gate: **x** - ``state.x(0)``
-  The Pauli-Y gate: **y** - ``state.y(0)``
-  The Pauli-Z gate: **z** - ``state.z(0)``
-  The CNOT gate: **cx** -
   ``state.cx(0, 1) # CNOT with control = 0, target = 1``
-  The SWAP gate: **swap** -
   ``state.swap(0,1) # Swaps the 0th and 1st qubit``
-  The Toffoli gate: **toffoli** -
   ``state.toffoli(0, 1, 2) # Toffoli with controls = (0, 1), target = 2``

These are all shorthand methods for the application of arbitrary gates.
For example, the application of a Hadamard gate above is shorthand for

.. code:: python

   import qcgpu

   h = qcgpu.gate.h()

   register = qcgpu.State(5)
   register.apply_gate(h, 0)

You can also use any of the gates as controlled gates. For example, the
application of the CNOT gate above is shorthand for

.. code:: python

   import qcgpu

   x = qcgpu.gate.x()

   register = qcgpu.State(5)
   register.apply_controlled_gate(x, 0, 1)

Applying A Gate To A Whole Register
----------------------------------

There is a convenience method to apply a gate to every qubit in the register.
The following applies a Hadamard gate to the whole register,

.. code:: python

   import qcgpu

   h = qcgpu.gate.h()

   register = qcgpu.State(5)
   register.apply_all(h)
   

User Defined Gates
------------------

Gates in QCGPU are represented by the ``qcgpu.Gate`` class.

The only gates that can be defined by the user are single qubit gates.

The process of creating a gate is

.. code:: python

   import qcgpu
   import numpy as np

   gate_matrix = np.array([
       [1, 0],
       [0, np.exp(1j * np.pi / 4)]
   ])

   gate = qcgpu.Gate(gate_matrix)

The input to the ``Gate`` constructor is checked to be a 2x2 unitary
matrix.

This newly created gate can then be applied the long hand way,

.. code:: python

   import qcgpu 

   register = qcgpu.State(2)
   register.apply_gate(gate, 0)