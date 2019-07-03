#include "tangle.h"

#include <stdio.h>
#include "mpi.h"

int main() {
  MPI_Init(NULL, NULL);

  count_to(10); 

  MPI_Finalize();

  return 0;
}