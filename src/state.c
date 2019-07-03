#include "mpi.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include "tangle.h"

TangleState create_state(int num_qubits, TangleEnvironment env) {
  TangleState state;

  state.num_qubits = num_qubits;
  state.rank = env.rank;
  state.nodes = env.nodes;
  state.k = log2(env.nodes);
  state.m = num_qubits - state.k;

  state.node_amps = 1LL << state.m;
  state.temp_amps = state.node_amps / 2;

  // Allocate the memory
  llint num_amplitudes = state.node_amps + state.temp_amps;
  state.amps = malloc(num_amplitudes * sizeof(cfloat));

  for (int i = 0; i < num_amplitudes; i++) {
    state.amps[i] = cfloat(0.0, 0.0);
  }

  if (state.rank == 0) {
    state.amps[0] = cfloat(1.0, 0.0);
  }

  MPI_Barrier(MPI_COMM_WORLD);

  return state;
}

void print_state(TangleState state) {
  for (int id = 0; id < state.nodes; id++) {
    if (state.rank == id) {
      for (llint i = 0; i < (1LL << state.m); i++) {
        cfloat amp = state.amps[i];
        printf("%lld: %f + i%f\n", state.rank * (1 << state.m) + i, creal(amp),
               cimag(amp));
      }
    }
    MPI_Barrier(MPI_COMM_WORLD);
  }
}