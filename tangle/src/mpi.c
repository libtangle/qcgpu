#include "tangle.h"

#include <stdio.h>
#include "mpi.h"

void count_to(int i)
{
    int rank;
    int size;

    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &size);

    printf("Hai from %d\n", rank);

    for (int j = 1; j <= i; j++)
    {
        if (j % size == rank)
        {
            printf("%d\n", j);
        }
        MPI_Barrier(MPI_COMM_WORLD);
    }
}