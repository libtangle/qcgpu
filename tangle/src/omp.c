#include "tangle.h"
#include <stdio.h>
#include <omp.h>

void count_to(int i)
{
#pragma omp parallel num_threads(8)
    {
        int i = omp_get_thread_num();

        for (int j = 1; j <= i; j++)
        {
            printf("MP %d: %d\n", i, j);
        }
    }
}