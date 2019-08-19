#include "mpi.h"
#include "tangle.h"

// A helper to deal with communication that goes above the
// communication size limits in OpenMPI. This could be removed
// by using some BigMPI implementation, but it's as easy to just
// do this.
void communication_helper(TangleState state, int node, llint idx_1, llint idx_2, int tag_1, int tag_2)
{
    // There is a limit to the amount of data that can be transferred.
    // This limit is 28 for a double, 27 for a long quad.
    llint message_size = 1LL << 29;

    // It will always be a state.temp_amps sized blocks transferred.
    // This is assuming the 'half and half' memory structure.
    // If needed, this can be decomposed.
    if (state.temp_amps < message_size)
    {
        message_size = state.temp_amps;
    }

    // Message size is a power of two, so breaking up the message can
    // be done without errors.
    for (llint i = 0; i < state.temp_amps / message_size; i++)
    {
        MPI_Sendrecv(&state.amps[idx_1 + (i * message_size)], message_size, MPI_COMPLEX, node, tag_1,
                     &state.amps[idx_2 + (i * message_size)], message_size, MPI_COMPLEX, node, tag_2,
                     MPI_COMM_WORLD, MPI_STATUS_IGNORE);
    }
}

// Send the top half of the state
// Receive into the temporary storage
void send_top(TangleState state, int node)
{
    communication_helper(state, node, 0, state.node_amps, 1, 2);
}

// Send the temporary storage
// Receive into the top half of the state
void receive_top(TangleState state, int node)
{
    communication_helper(state, node, state.node_amps, 0, 3, 4);
}

// Send the bottom half of the state
// Receive into the temporary state
void send_bottom(TangleState state, int node)
{
    communication_helper(state, node, state.temp_amps, state.node_amps, 2, 1);
}

// Send the temporary storage
// Receive into the bottom half of the state
void receive_bottom(TangleState state, int node)
{
    communication_helper(state, node, state.node_amps, state.temp_amps, 4, 3);
}
