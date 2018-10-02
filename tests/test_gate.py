import pytest
from qcgpu.gate import Gate, h, x, y, z, s, t, sqrt_x
import numpy as np

def test_gate_creation():
    Gate(np.array([[0, 1], [1, 0]])) # A clearly unitary gate
    h()
    x()
    y()
    z()
    s()
    t()
    sqrt_x()

def test_using_list():
    return Gate([[1, 0], [0, 1]])

def test_non_unitary_gate_creation_fails():
    # A clearly non unitary gate
    with pytest.raises(Exception):
        return Gate(np.array([[12, 33], [-7j, 1]]))

def test_large_gate_creation_fails():
    # A gate that is not 2x2
    with pytest.raises(Exception):
        return Gate(np.ones(4))

def test_using_scalar_fails():
    with pytest.raises(Exception):
        return Gate(2)

def test_using_string_fails():
    with pytest.raises(Exception):
        return Gate("this should fail")
