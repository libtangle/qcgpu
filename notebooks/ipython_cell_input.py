def bench_qcgpu(n, depth):
    state = qcgpu.State(n)

    h = qcgpu.gate.h()
    x = qcgpu.gate.x()
    sqrt_x = qcgpu.gate.sqrt_x()

    print('started')
    start = time.time()

    for level in range(depth):
        for q in range(n):
    
            state.apply_gate(h, q)
            state.apply_gate(sqrt_x, q)

            if q != 0:
                state.apply_controlled_gate(x, q, 0)
        
    runtime = time.time() - start
    print('ended: ', runtime)
    return runtime

bench_qcgpu(26,5)