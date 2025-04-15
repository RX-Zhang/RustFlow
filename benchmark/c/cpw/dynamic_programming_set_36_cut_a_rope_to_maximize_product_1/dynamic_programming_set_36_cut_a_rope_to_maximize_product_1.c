#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int dynamic_programming_set_36_cut_a_rope_to_maximize_product_1 ( int n ) ;
int dynamic_programming_set_36_cut_a_rope_to_maximize_product_1 ( int n ) {
  if ( n == 2 || n == 3 ) return ( n - 1 );
  int res = 1;
  while ( n > 4 ) {
    n -= 3;
    res *= 3;
  }
  return ( n * res );
}


