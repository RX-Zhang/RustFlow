

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int square_pyramidal_number_sum_squares ( int s ) ;
int square_pyramidal_number_sum_squares ( int s ) {
  int sum = 0;
  for ( int n = 1;
  sum < s;
  n ++ ) {
    sum += n * n;
    if ( sum == s ) return n;
  }
  return - 1;
}


