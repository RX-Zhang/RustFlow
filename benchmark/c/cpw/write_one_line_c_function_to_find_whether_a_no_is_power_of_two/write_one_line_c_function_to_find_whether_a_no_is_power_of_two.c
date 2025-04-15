

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int write_one_line_c_function_to_find_whether_a_no_is_power_of_two ( int n ) ;
int write_one_line_c_function_to_find_whether_a_no_is_power_of_two ( int n ) {
  if ( n == 0 ) return 0;
  while ( n != 1 ) {
    if ( n % 2 != 0 ) return 0;
    n = n / 2;
  }
  return 1;
}


