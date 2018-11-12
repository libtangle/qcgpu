import numpy as np
import functools

def memoize(func):
    cache = func.cache = {}

    @functools.wraps(func)
    def memoized_func():
        key = 'a'
        if key not in cache:
            cache[key] = func()
        return cache[key]

    return memoized_func

class Gate:
    def __init__(self, gate, unitary=True):
        gate = np.array(gate)
        
        if gate.shape != (2, 2):
            raise ValueError(
                "Gate is not a 2x2 matrix. " + 
                "For larger gates, please decompose into 2x2 matrices " +
                "and/or use the controlled gate functionality."
            )

        # Check the gate is unitary
        if unitary:
            if (not np.allclose(np.eye(gate.shape[0]), np.dot(gate.conjugate().transpose(), gate))):
                raise ValueError("gate is not unitary.")

        self.a = complex(gate[0, 0])
        self.b = complex(gate[0, 1])
        self.c = complex(gate[1, 0])
        self.d = complex(gate[1, 1])

    def __repr__(self):
        return '[{:.4f}, {:.4f}]\n[{:.4f}, {:.4f}]'.format(self.a, self.b, self.c, self.d)

@memoize
def h():
    return Gate(np.array([[1, 1], [1, -1]]) / np.sqrt(2))

@memoize
def x():
    return Gate(np.array([[0, 1], [1, 0]]))

@memoize
def y():
    return Gate(np.array([[0, -1j], [1j, 0]]))

@memoize
def z():
    return Gate(np.array([[1, 0], [0, -1]]))

@memoize
def s():
    return Gate(np.array([[1, 0], [0, 1j]]))

@memoize
def t():
    return Gate(np.array([[1, 0], [0, np.exp(np.pi * 1j / 4)]]))

@memoize
def sqrt_x():
    return Gate(0.5 * np.array([[1+1j, 1-1j], [1-1j, 1+1j]]))

