

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int number_non_negative_integral_solutions_b_c_n ( int n ) ;
int number_non_negative_integral_solutions_b_c_n ( int n ) {
  int result = 0;
  for ( int i = 0;
  i <= n;
  i ++ ) for ( int j = 0;
  j <= n - i;
  j ++ ) for ( int k = 0;
  k <= ( n - i - j );
  k ++ ) if ( i + j + k == n ) result ++;
  return result;
}


