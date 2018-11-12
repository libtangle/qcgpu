import qcgpu

# 3 qubits, f(x) = x_0 NOT x_1 x_2
# Balanced
balanced_state = qcgpu.State(3)

balanced_state.apply_all(qcgpu.gate.h())

# Oracle U_f
balanced_state.h(2)
balanced_state.z(0)
balanced_state.cx(1, 2)
balanced_state.h(2)

balanced_state.apply_all(qcgpu.gate.h())

outcomes = balanced_state.measure(samples = 1000)

if int(max(outcomes, key=outcomes.get)) == 0:
    print('constant')
else:
    print('balanced')


# 3 qubits, f(x) = 0
# Constant
constant_state = qcgpu.State(3)

constant_state.apply_all(qcgpu.gate.h())

# Oracle is equivalent to the identity gate, 
# thus has no effect on the state

constant_state.apply_all(qcgpu.gate.h())

outcomes = constant_state.measure(samples = 1000)

if int(max(outcomes, key=outcomes.get)) == 0:
    print('constant')
else:
    print('balanced')