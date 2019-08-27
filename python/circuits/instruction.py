"""
The class defined here describes all
possible instructions that can occur.
This includes:

- Adding quantum gates
- Measurement (including sampling, etc.)
- Reset
- Initialization

The instructions defined here are independent
of backend, and each of the backends should
implement their own validation step, which
should also correspond to the backend selection
procedure.
"""


# An instruction consists of a number of parts

class Instruction:
    def __init__(self, name, num_qubits, num):
        pass


class Gate(Instruction):
    def __init__(self):
        pass
