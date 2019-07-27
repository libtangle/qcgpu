#include "tangle.h"
#include <stdio.h>
// #include "mpi.h"

int main()
{
    // MPI_Init(NULL, NULL);

    printf("Welcome\n");

    count_to(10);

    // MPI_Finalize();

    return 0;
}

// #include <stdio.h>
// #include <omp.h>

// int main()
// {
// #pragma omp parallel num_threads(3)
//     {
//         int id = omp_get_thread_num();
//         int data = id;
//         int total = omp_get_num_threads();
//         printf("Greetings from process %d out of %d with Data %d\n", id, total, data);
//     }
//     printf("parallel for ends.\n");
//     return 0;
// }