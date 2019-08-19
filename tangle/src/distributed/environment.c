#include "tangle.h"
#include "mpi.h"

TangleEnvironment initialize_environment()
{
    TangleEnvironment env;

    int rank, nodes, initialized;
    MPI_Initialized(&initialized);

    if (!initialized)
    {
        MPI_Init(0, 0);
    }

    MPI_Comm_size(MPI_COMM_WORLD, &nodes);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    env.rank = rank;
    env.nodes = nodes;

    return env;
}

void destroy_environment(TangleEnvironment env)
{
    int finalized;
    MPI_Finalized(&finalized);

    if (!finalized)
    {
        MPI_Finalize();
    }
}