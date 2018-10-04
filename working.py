import qcgpu
import time

s = qcgpu.State(28)
h = qcgpu.gate.h()

s.apply_all(h)



print(s.measure(1000))
# print(s.backend.measure(samples=10000))