#include "tangle.h"

#include <stdio.h>

void say_hi() {
  printf("Hi!\n");

#ifdef _DO_THING
  printf("YOU ENTERED POWER MODE!!!1!!\n");
#endif

#ifdef _DO_OTHER
  printf("YOU ENTERED OTHER MODE!!!1!!\n");
#endif
}