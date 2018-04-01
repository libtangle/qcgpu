import time
import sys
from qiskit import QuantumProgram

MILLIS = 1000
MICROS = MILLIS * 1000
NANOS = MICROS * 1000

def benchmark():
    argument = int(sys.argv[1])

    for line in sys.stdin:
        iters = int(line.strip())

        # Setup



        start = time.perf_counter()
        i = 0
        for x in range(iters):
            qp = QuantumProgram()
            qr = qp.create_quantum_register('qr',argument)
            cr = qp.create_classical_register('cr',argument)
            qc = qp.create_circuit('Bell',[qr],[cr])
            for i in range(argument):
                qc.h(qr[i])
            for i in range(argument):
                qc.measure(qr[i], cr[i])
            result = qp.execute('Bell')
            result.get_counts('Bell')
        end = time.perf_counter()

        # Teardown

        delta = end - start
        nanos = int(delta * NANOS)
        print("%d" % nanos)
        sys.stdout.flush()

benchmark()
