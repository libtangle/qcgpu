=================
Quantum Registers
=================

QCGPU provides a class to represent the register, ``qcgpu.State``. The
register class stores (on the OpenCL device) a state vector. This state
vector is a chunk of memory, the size of which is:

.. math::


   64 \cdot 2^n \text{ bits}.

This means you would need just 2kb of memory to have a 5 qubit register,
a 30 qubit register would take up 9gb of memory.

This is something to be aware of, as the state vector must fit in the
memory of the device you wish to use.

Using the ``State`` class
-------------------------

To create a new register, you can use

.. code:: python

   import qcgpu

   register = qcgpu.State(5)

This will create a 5 qubit register.

When you run this, you may be prompted to choose a device. This is
normal, as you can have more than 1 device that supports OpenCL in your
computer. Just choose the one you want.

Mathematical Description
-------------------------

This class represents a state vector :math:`\lvert \psi \rangle` with

.. math::


   \lvert \psi \rangle = \sum_{j = 0}^{2^n - 1} \alpha_j \lvert j \rangle

where :math:`n` is the number of qubits, :math:`\alpha_j` is the
amplitude and the state is :math:`j` runs overall :math:`2^n` basis
states.