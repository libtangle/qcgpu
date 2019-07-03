#include <stdio.h>
#include "mpi.h"
#include "tangle.h"

int main() {
  // Create the environment
  TangleEnvironment env = initialize_environment();

  int num_qubits = 24;
  TangleState state = create_state(num_qubits, env);

  for (int i = 0; i < num_qubits; i++) {
    X(state, i);
  }

  llint result = measure(state);
  

  if (state.rank == 0) {
    printf("Result: %lld\n", result);
  }

  // Finish the computation
  destroy_environment(env);

  return 0;
}