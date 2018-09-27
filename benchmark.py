# This script is to run the benchmarks.
import numpy as np
import sys
import csv
import time
import qcgpu
import os.path

results_file = 'benchmark_results.csv'

def bench_qcgpu(n, depth):
    state = qcgpu.State(n)

    h = qcgpu.gate.h()
    x = qcgpu.gate.x()
    sqrt_x = qcgpu.gate.sqrt_x()

    start = time.time()

    for level in range(depth):
        for q in range(n):
    
            state.apply_gate(h, q)
            state.apply_gate(sqrt_x, q)

            if q != 0:
                state.apply_controlled_gate(x, q, 0)
        
    runtime = time.time() - start
    return {'name': 'qcgpu', 'num_qubits': n, 'depth': depth, 'time': runtime}

def create_csv(filename):
    file_exists = os.path.isfile(filename)
    csvfile = open(filename, 'a')
   
    headers = ['name', 'num_qubits', 'depth', 'time']
    writer = csv.DictWriter(csvfile, delimiter=',', lineterminator='\n',fieldnames=headers)

    if not file_exists:
        writer.writeheader()  # file doesn't exist yet, write a header

    # writer.writerow({'TimeStamp': dic['ts'], 'light': dic['light'], 'Proximity': dic['prox']}

    return writer

def write_csv(writer, data):
    print("Qubits: " + str(data['num_qubits']) + ", Depth: " + str(data['depth']) + ", Time: " + str(data['depth']))
    writer.writerow(data)

writer = create_csv(results_file)
for i in range(2, 25):
    for d in [5,10,15,20]:
        data = bench_qcgpu(i, d)
        write_csv(writer, data)


