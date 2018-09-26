from backend import Backend
import pyopencl as cl
import numpy as np

class State:
    """Represents the state of a quantum register"""
    def __init__(self, num_qubits):
        if not isinstance(num_qubits, int):
            raise ValueError("num_qubits must be an int")
        if num_qubits <= 0:
            raise ValueError("num_qubits must be a positive integer")

        self.num_qubits = num_qubits
        self.backend = Backend(num_qubits)

    def apply_gate(self, gate, target):
        if not isinstance(target, int) or target < 0:
            raise ValueError("target must be an int > 0")

        # TODO: Check that gate is correct

        self.backend.apply_gate(gate, target)


    def apply_all(self, gate):
        # TODO: Check that gate is correct
        for i in range(self.num_qubits):
            self.apply_gate(gate, i)

    def apply_controlled_gate(self, gate, control, target):
        if not isinstance(target, int) or target < 0:
            raise ValueError("target must be an int > 0")
        
        if not isinstance(control, int) or control < 0:
            raise ValueError("control must be an int > 0")

        # TODO: Check that gate is correct

        self.backend.apply_controlled_gate(gate, control, target)

    def amplitudes(self):
        return self.backend.amplitudes()
    
    def probabilities(self):
        return self.backend.probabilities()

    def flush(self):
        self.backend.release()

    def __repr__(self):
        """A string representation of the state"""

        # TODO: Finish this method
        return np.array_str(self.backend.buffer)