#include "mpi.h"
#include "tangle.h"
#include <complex.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

llint measure(TangleState state) {
  float sample_value = 1.0;
  llint measured = -1;

  if (state.rank == 0) {
    // The master node selects a random value between 0 and 1
    srand(time(0));
    sample_value = (float)rand() / (float)RAND_MAX;

    // Then, the master node goes through it's own amplitudes, to
    // attempt to select a measurment outcome
    llint i;
    for (i = 0; i < state.node_amps && sample_value > 0; i++) {
      float amp_len = cabs(state.amps[i]);
      sample_value = sample_value - (amp_len * amp_len);
    }

    // We now have to send the reduced random value to the other nodes
    if (state.nodes > 1) {
      MPI_Send(&sample_value, 1, MPI_REAL, 1, 5, MPI_COMM_WORLD);
    }

    // If the sample value is <= 0, then we have a measurment outcome.
    // No other nodes will send a measurment value if sample value <= 0
    if (sample_value <= 0) {
      measured = state.rank * (1 << state.m) + (i - 1);
    } else {
      // Otherwise, we have to look on other nodes
      MPI_Recv(&measured, 1, MPI_LONG_LONG_INT, MPI_ANY_SOURCE, 6,
               MPI_COMM_WORLD, MPI_STATUS_IGNORE);
    }
  } else {
    // This is not a master node, so we need to wait for the previous reduced
    // random value
    MPI_Recv(&sample_value, 1, MPI_REAL, state.rank - 1, 5, MPI_COMM_WORLD,
             MPI_STATUS_IGNORE);

    // Attempt to reduce the random value if needed
    if (sample_value > 0) {
      llint i;
      for (i = 0; i < state.node_amps && sample_value > 0; i++) {
        float amp_len = cabs(state.amps[i]);
        sample_value = sample_value - (amp_len * amp_len);
      }

      // If the sample_value is now <= 0, then we have a measurment value, which
      // can be communicated
      if (sample_value <= 0) {
        measured = state.rank * (1 << state.m) + (i - 1);
        MPI_Send(&measured, 1, MPI_LONG_LONG_INT, 0, 6, MPI_COMM_WORLD);
      }
    }

    // Again, the reduced random value can be passed on
    if ((state.rank + 1) < state.nodes) {
      MPI_Send(&sample_value, 1, MPI_REAL, state.rank + 1, 5, MPI_COMM_WORLD);
    }
  }

  return measured;
}
