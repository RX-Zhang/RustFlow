#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_digits_factorial_set_1 ( int n ) ;
int count_digits_factorial_set_1 ( int n ) {
  if ( n < 0 ) return 0;
  if ( n <= 1 ) return 1;
  double digits = 0;
  for ( int i = 2;
  i <= n;
  i ++ ) digits += log10 ( i );
  return floor ( digits ) + 1;
}


