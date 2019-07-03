#include "mpi.h"
#include "tangle.h"

void communication_helper(TangleState state, int node, llint idx_1, llint idx_2,
                          int tag_1, int tag_2) {
  // There is a limit to the amount of data that can be transfered
  llint message_size = 1LL << 29; // 28 for a double, 27 for a long quad
  
  // It will always be a state.temp_amps sized block transfered.
  // This can be decomposed if needed
  if (state.temp_amps < message_size) {
    message_size = state.temp_amps;
  }

  // Message size is a power of two, so we can break up the message

  for (llint i = 0; i < state.temp_amps / message_size; i++) {
    MPI_Sendrecv(&state.amps[idx_1 + (i * message_size)], message_size, MPI_COMPLEX, node, tag_1,
               &state.amps[idx_2 + (i * message_size)], message_size, MPI_COMPLEX, node, tag_2,
               MPI_COMM_WORLD, MPI_STATUS_IGNORE);
  }

  
}

// Send the top half of the state,
// Recieve into the temporary storage
void send_top(TangleState state, int node) {
  communication_helper(state, node, 0, state.node_amps, 1, 2);
}

// Send the temporary storage,
// recieve into the top half
void recieve_top(TangleState state, int node) {
  communication_helper(state, node, state.node_amps, 0, 3, 4);
}

// Send the bottom half of the state
// Recieve into the temporary state
void send_bottom(TangleState state, int node) {
  communication_helper(state, node, state.temp_amps, state.node_amps, 2, 1);
}

// Send the temporary storage
// Receive into the bottom half
void recieve_bottom(TangleState state, int node) {
  communication_helper(state, node, state.node_amps, state.temp_amps, 4, 3);
}