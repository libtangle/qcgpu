===============
Getting Started
===============

When using this library, you will most likely be using the :class:`~qiskit.State` class.
This class represents the state of a quantum register. 
Using this class you can apply gates to the register, measure, get the state vector and things like that.

To run a simple quantum circuit, you can use something like this,

.. code-block:: python

    # Import QCGPU
    import qcgpu

    # Create a new quantum register with 2 qubits
    register = qcgpu.State(2)

    # Apply a hadamard (H) gate to the first qubit.
    # You should note that the qubits are zero indexed
    register.h(0)

    # Add a controlled not (CNOT/CX) gate, with the control as
    # the first qubit and target as the second.
    # The register will now be in the bell state.
    register.cx(0, 1)

    # Perform a measurement with 1000 samples
    results = register.measure(samples=1000)

    # Show the results
    print(results)

The output of a measurement gives a dictionary of measurement outcomes,
along with how often they occurred.

.. code-block:: python

    {'00': 486, '11': 514}

There are a few different ways to do things using QCGPU, 
so you should check out the rest of the documentation too