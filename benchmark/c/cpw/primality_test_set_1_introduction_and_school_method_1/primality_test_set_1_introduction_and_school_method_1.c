

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int primality_test_set_1_introduction_and_school_method_1 ( int n ) ;
int primality_test_set_1_introduction_and_school_method_1 ( int n ) {
  if ( n <= 1 ) return 0;
  if ( n <= 3 ) return 1;
  if ( n % 2 == 0 || n % 3 == 0 ) return 0;
  for ( int i = 5;
  i * i <= n;
  i = i + 6 ) if ( n % i == 0 || n % ( i + 2 ) == 0 ) return 0;
  return 1;
}


