

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int triangular_numbers ( int num ) ;
int triangular_numbers ( int num ) {
  if ( num < 0 ) return 0;
  int sum = 0;
  for ( int n = 1;
  sum <= num;
  n ++ ) {
    sum = sum + n;
    if ( sum == num ) return 1;
  }
  return 0;
}


