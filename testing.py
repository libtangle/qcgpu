import qcgpu

state = qcgpu.State(2)
h = qcgpu.gate.h()
x = qcgpu.gate.x()

state.apply_gate(x, 0)
state.apply_controlled_gate(x, 0)

print(state)
