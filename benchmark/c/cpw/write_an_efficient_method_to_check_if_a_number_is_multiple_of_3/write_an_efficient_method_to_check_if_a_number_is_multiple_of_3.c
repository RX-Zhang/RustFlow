

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int write_an_efficient_method_to_check_if_a_number_is_multiple_of_3 ( int n ) ;
int write_an_efficient_method_to_check_if_a_number_is_multiple_of_3 ( int n ) {
  int odd_count = 0;
  int even_count = 0;
  if ( n < 0 ) n = - n;
  if ( n == 0 ) return 1;
  if ( n == 1 ) return 0;
  while ( n ) {
    if ( n & 1 ) odd_count ++;
    if ( n & 2 ) even_count ++;
    n = n >> 2;
  }
  return write_an_efficient_method_to_check_if_a_number_is_multiple_of_3 ( abs ( odd_count - even_count ) );
}


