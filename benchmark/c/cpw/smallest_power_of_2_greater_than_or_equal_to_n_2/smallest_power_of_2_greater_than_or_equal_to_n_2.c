

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int smallest_power_of_2_greater_than_or_equal_to_n_2 ( unsigned int n ) ;
unsigned int smallest_power_of_2_greater_than_or_equal_to_n_2 ( unsigned int n ) {
  n --;
  n |= n >> 1;
  n |= n >> 2;
  n |= n >> 4;
  n |= n >> 8;
  n |= n >> 16;
  n ++;
  return n;
}


