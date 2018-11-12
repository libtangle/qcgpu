"""
Quantum Register Object
"""
import qcgpu
from qcgpu.backend import Backend
import pyopencl as cl
import numpy as np

class State:
    """A class for representing quantum registers.

    The State class is the QCGPU representation of a quantum 
    register / state vector.

    This class is what should be used to perform the simulations,
    and has method for things such as applying gates, measurements,
    getting probabilities and such.

    As QCGPU uses OpenCL, you may be queried about which OpenCL device
    to use. This will only happen when running things such a python repl,
    or running a script using QCGPU from the command line. Otherwise, a
    device will be chosen heuristically.

    When the register is created, it will be left in the state

    .. math::

        \\lvert 000 \\dots 0 \\rangle

    With the given number of qubits.

    Args:
        num_qubits (int): The number of qubits to create in the register.  
            This must be greater then zero.

    Returns:
        State: A representation of the quantum register.

    Examples
        >>> qcgpu.State(3)
            Choose platform:
            [0] <pyopencl.Platform 'NVIDIA CUDA' at 0x2f22390>
            Choice [0]:0
            Set the environment variable PYOPENCL_CTX='0' to avoid being asked again.
            [[array(1.+0.j, dtype=complex64)]
             [array(0.+0.j, dtype=complex64)]
             [array(0.+0.j, dtype=complex64)]
             [array(0.+0.j, dtype=complex64)]]
    """
    def __init__(self, num_qubits):
        
        if not isinstance(num_qubits, int):
            raise ValueError("num_qubits must be an int")
        if num_qubits <= 0:
            raise ValueError("num_qubits must be a positive integer")

        #: The number of qubits that are in the register
        self.num_qubits = num_qubits
        self.backend = Backend(num_qubits)

    def apply_gate(self, gate, target):
        """Applies a single qubit unitary gate to the register.

        Args:
            gate (~qcgpu.Gate): The gate to be applied to the register
            target (int): The index of the qubit in the register that the gate
                is to be applied to.
        """
        if not isinstance(target, int) or target < 0:
            print(target)
            raise ValueError("target must be an int >= 0")

        # TODO: Check that gate is correct

        self.backend.apply_gate(gate, target)

    def apply_all(self, gate):
        # TODO: Check that gate is correct
        for i in range(self.num_qubits):
            self.apply_gate(gate, i)

    def apply_controlled_gate(self, gate, control, target):
        if not isinstance(target, int) or target < 0:
            raise ValueError("target must be an int >= 0")
        
        if not isinstance(control, int) or control < 0:
            raise ValueError("control must be an int >= 0")

        # TODO: Check that gate is correct

        self.backend.apply_controlled_gate(gate, control, target)
    
    def apply_controlled_controlled_gate(self, gate, control, control2, target):
        if not isinstance(target, int) or target < 0:
            raise ValueError("target must be an int >= 0")
        
        if not isinstance(control, int) or control < 0:
            raise ValueError("control must be an int >= 0")

        if not isinstance(control2, int) or control2 < 0:
            raise ValueError("control must be an int >= 0")

        # TODO: Check that gate is correct

        self.backend.apply_controlled_controlled_gate(gate, control, control2, target)

    def measure_qubit(self, target, samples=1):
        return self.backend.measure_qubit(target, samples)

    def measure_collapse(self, target):
        return self.backend.measure_collapse(target)

    def measure(self, samples=1):
        return self.backend.measure(samples)

    def measure_first(self, num=1, samples=1):
        return self.backend.measure_first(num, samples)

    def amplitudes(self):
        return self.backend.amplitudes()[0]
    
    def probabilities(self):
        return self.backend.probabilities()[0]

    def __repr__(self):
        """A string representation of the state"""

        # TODO: Finish this method
        return np.array_str(self.backend.buffer)


    # Gates
    def h(self, target):
        self.apply_gate(qcgpu.gate.h(), target)

    def x(self, target):
        self.apply_gate(qcgpu.gate.x(), target)

    def y(self, target):
        self.apply_gate(qcgpu.gate.y(), target)

    def z(self, target):
        self.apply_gate(qcgpu.gate.z(), target)

    def s(self, target):
        self.apply_gate(qcgpu.gate.s(), target)

    def t(self, target):
        self.apply_gate(qcgpu.gate.t(), target)

    def sqrt_x(self, target):
        self.apply_gate(qcgpu.gate.sqrt_x(), target)

    def cx(self, control, target):
        self.apply_controlled_gate(qcgpu.gate.x(), control, target)

    def cnot(self, control, target):
        self.apply_controlled_gate(qcgpu.gate.x(), control, target)

    def toffoli(self, control, control2, target):
        self.apply_controlled_controlled_gate(qcgpu.gate.x(), control, control2, target)

    def u(self, target, theta, phi, lda):
        a = np.exp(-1j * (phi + lda) / 2) * np.cos(theta / 2)
        b = - np.exp(-1j * (phi - lda) / 2) * np.sin(theta / 2)
        c = np.exp(1j * (phi - lda) / 2) * np.sin(theta / 2)    
        d = np.exp(1j * (phi + lda) / 2) * np.cos(theta / 2)
    
        gate_matrix = np.array([
            [a, b],
            [c, d]
        ])
        self.apply_gate(qcgpu.Gate(gate_matrix), target)
    
    def u1(self, target, lda):
        self.u(target, 0, 0, lda)

    def u2(self, target, phi, lda):
        self.u(target, np.pi / 2, phi, lda)

    def u3(self, target, theta, phi, lda):
        self.u(target, theta, phi, lda)

    def cu(self, control, target, theta, phi, lda):
        a = np.exp(-1j * (phi + lda) / 2) * np.cos(theta / 2)
        b = - np.exp(-1j * (phi - lda) / 2) * np.sin(theta / 2)
        c = np.exp(1j * (phi - lda) / 2) * np.sin(theta / 2)    
        d = np.exp(1j * (phi + lda) / 2) * np.cos(theta / 2)
    
        gate_matrix = np.array([
            [a, b],
            [c, d]
        ])
        self.apply_controlled_gate(qcgpu.Gate(gate_matrix), control, target)


    def cu1(self, control, target, lda):
        self.cu(control, target, 0, 0, lda)

    def cu2(self, control, target, phi, lda):
        self.cu(control, target, np.pi / 2, phi, lda)

    def cu3(self, control, target, theta, phi, lda):
        self.cu(control, target, theta, phi, lda)