

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int squared_triangular_number_sum_cubes ( int s ) ;
int squared_triangular_number_sum_cubes ( int s ) {
  int sum = 0;
  for ( int n = 1;
  sum < s;
  n ++ ) {
    sum += n * n * n;
    if ( sum == s ) return n;
  }
  return - 1;
}


