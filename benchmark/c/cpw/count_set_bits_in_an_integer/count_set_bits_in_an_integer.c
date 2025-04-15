#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

unsigned int count_set_bits_in_an_integer ( unsigned int n ) ;
unsigned int count_set_bits_in_an_integer ( unsigned int n ) {
  unsigned int count = 0;
  while ( n ) {
    count += n & 1;
    n >>= 1;
  }
  return count;
}


