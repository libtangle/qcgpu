from qcgpu import State
import pytest

def test_state_creation():
    # Any machine should be able to handle 14 qubits
    for i in range(1, 15):
        State(i)

def test_state_with_no_qubits_fails():
    with pytest.raises(Exception):
        State(0)

