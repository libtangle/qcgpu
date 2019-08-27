"""
This file contains the main circuit implementation/representation class
"""


class Instruction:
    def __init__(self, name, qubits, parameters=None):
        self.name = name
        self.qubits = qubits
        self.parameters = parameters

    def __repr__(self):
        if self.parameters:
            return "{} ({}) {};".format(self.name, self.parameters, self.qubits)
        return "{} {};".format(self.name, self.qubits)


class Circuit:
    """
    Represents a Quantum Circuit. 

    The number of qubits is inferred from 
    the instructions provided
    """

    def __init__(self):
        self.num_qubits = 0

        # Represents the instructions that are given to
        # the quantum circuit
        self.instructions = []

        # Represent the numerical remapping in the circuit,
        # to avoid badly labelled registers.
        self.idx_map = {}

    # TODO:
    # - Mirror
    # - Invert
    # - Combine multiple circuits
    # - Multiple register handling

    def add_instruction(self, gate_name, *qubits):
        # instruction
        pass

    def __repr__(self):
        return str(vars(self))


if __name__ == "__main__":
    circuit = Circuit(3)
    print(circuit)

    h = Instruction('h', [0])
    print(h)
