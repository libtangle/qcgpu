Quantum Operations
==================

There are a number of operations you can perform on quantum registers
with QCGPU.

Measurement
-----------

Measurement of a register in QCGPU doesn’t collapse the state (although
this may be added in the future). When you measure the state, you can
specify the number of times to sample. The output of this ``measure``
function is a dictionary with the bitstrings of the outputs, along with
the number of times they were measured.

You can measure a register as follows,

.. code:: python

   import qcgpu

   register = qcgpu.State(5)

   register.measure(samples=1000)
   # {'00000': 1000}

There is also a convenience method to measure only a single qubit.
Again, the state is not collapsed

.. code:: python

   import qcgpu

   register = qcgpu.State(5)

   register.h(0)

   register.measure_qubit(0, samples=1000)
   # {'1': 523, '0': 477}

Probability
-----------

QCGPU provides another method for getting the probability of each
outcome.

The probability of getting an outcome :math:`j` from a state
:math:`\lvert \psi \rangle = \sum_{j = 0}^{2^n - 1} \alpha_j \lvert j \rangle`
is

.. math::


   P(j) = \lvert \alpha_j \lvert^2

The method ``probabilities`` returns a numpy array with each of the
values corresponding to :math:`\lvert \alpha_j \lvert ^2` for each index
:math:`j`.

.. code:: python

   import qcgpu

   register = qcgpu.State(1)
   register.h(0)

   register.probabilities() # [0.5, 0.5]

This method is particularly useful to plot the probabilities of each
outcome.