// #include "tangle.h"
// #include <stdio.h>
// // #include "mpi.h"

// int main()
// {
//     // MPI_Init(NULL, NULL);

//     printf("Welcome\n");

//     count_to(10);

//     // MPI_Finalize();

//     return 0;
// }

// // #include <stdio.h>
// // #include <omp.h>

// // int main()
// // {
// // #pragma omp parallel num_threads(3)
// //     {
// //         int id = omp_get_thread_num();
// //         int data = id;
// //         int total = omp_get_num_threads();
// //         printf("Greetings from process %d out of %d with Data %d\n", id, total, data);
// //     }
// //     printf("parallel for ends.\n");
// //     return 0;
// // }

#include <stdio.h>
#include <stdlib.h>
#include "tangle.h"

int main(int argc, char *argv[])
{
    TangleEnvironment env = initialize_environment();

    if (env.rank == 0 && argc <= 1)
    {
        printf("Please enter the number of qubits.\n");
        exit(1);
    }

    int num_qubits = atoi(argv[1]);

    TangleState state = create_state(num_qubits, env);

    // Apply a hadamard gate to each qubit
    // for (int i = 0; i < num_qubits; i++)
    // {
    //     H(state, i);
    // }
    H(state, 0);

    // Print the initial amplitude
    if (env.rank == 0)
    {
        cfloat amp = state.amps[0];
        printf("|0> = %f + i%f\n", creal(amp), cimag(amp));
    }

    // Finish the computation
    destroy_environment(env);

    return 0;
}