

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int find_sum_even_factors_number ( int n ) ;
int find_sum_even_factors_number ( int n ) {
  if ( n % 2 != 0 ) return 0;
  int res = 1;
  for ( int i = 2;
  i <= sqrt ( n );
  i ++ ) {
    int count = 0, curr_sum = 1, curr_term = 1;
    while ( n % i == 0 ) {
      count ++;
      n = n / i;
      if ( i == 2 && count == 1 ) curr_sum = 0;
      curr_term *= i;
      curr_sum += curr_term;
    }
    res *= curr_sum;
  }
  if ( n >= 2 ) res *= ( 1 + n );
  return res;
}


