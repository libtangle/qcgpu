import qcgpu
from qcgpu import State
import pytest
import numpy as np

def test_application_x():
    s = State(3)

    x = qcgpu.gate.x()
    s.apply_gate(x, 0)

    res = np.array([0,1,0,0,0,0,0,0]).astype(np.complex64).transpose()
    amps = s.amplitudes()

    assert np.allclose(res, amps)

def test_apply_all_x():
    s = State(3)

    x = qcgpu.gate.x()
    s.apply_all(x)

    res = np.array([0,0,0,0,0,0,0,1]).astype(np.complex64).transpose()
    amps = s.amplitudes()

    assert np.allclose(res, amps)

def test_application_h():
    s = State(3)

    h = qcgpu.gate.h()
    s.apply_gate(h, 1)

    res = (1/np.sqrt(2)) * np.array([1,0,1,0,0,0,0,0]).astype(np.complex64).transpose()
    amps = s.amplitudes()

    assert np.allclose(res, amps)

def test_apply_all_h():
    s = State(8)

    h = qcgpu.gate.h()
    s.apply_all(h)

    res = (1 / np.sqrt(2**8)) * np.ones((1, 2**8), dtype=np.complex64)
    amps = s.amplitudes()

    assert np.allclose(res, amps)

def test_apply_cnot_1():
    s = State(2)

    x = qcgpu.gate.x()
    s.apply_controlled_gate(x, 0, 1)

    res = np.array([1,0,0,0]).astype(np.complex64).transpose()
    amps = s.amplitudes()

    assert np.allclose(res, amps)

def test_apply_cnot_2():
    s = State(2)

    x = qcgpu.gate.x()
    s.apply_gate(x, 0)
    s.apply_controlled_gate(x, 0, 1)

    res = np.array([0,0,0,1]).astype(np.complex64).transpose()
    amps = s.amplitudes()

    assert np.allclose(res, amps)