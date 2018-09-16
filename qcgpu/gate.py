import numpy as np

class Gate:
    def __init__(self, gate):
        if gate.shape != (2, 2):
            raise ValueError(
                "Gate is not a 2x2 matrix. " + 
                "For larger gates, please decompose into 2x2 matrices " +
                "and/or use the controlled gate functionality."
            )

        # Check the gate is unitary
        if (not np.allclose(np.eye(gate.shape[0]), gate.H * gate)):
            raise ValueError("gate is not unitary.")

        self.a = complex(gate[0, 0])
        self.b = complex(gate[0, 1])
        self.c = complex(gate[1, 0])
        self.d = complex(gate[1, 1])

    def __repr__(self):
        return '[{:.4f}, {:.4f}]\n[{:.4f}, {:.4f}]'.format(self.a, self.b, self.c, self.d)

def h():
    return Gate(np.matrix([[1, 1], [1, -1]]) / np.sqrt(2))

def x():
    return Gate(np.matrix([[0, 1], [1, 0]]))

def y():
    return Gate(np.matrix([[0, -1j], [1j, 0]]))

def z():
    return Gate(np.matrix([[1, 0], [0, -1]]))
