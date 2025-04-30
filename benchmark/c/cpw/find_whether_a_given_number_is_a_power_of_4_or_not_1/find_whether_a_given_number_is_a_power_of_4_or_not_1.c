

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int find_whether_a_given_number_is_a_power_of_4_or_not_1 ( unsigned int n ) ;
int find_whether_a_given_number_is_a_power_of_4_or_not_1 ( unsigned int n ) {
  int count = 0;
  if ( n && ! ( n & ( n - 1 ) ) ) {
    while ( n > 1 ) {
      n >>= 1;
      count += 1;
    }
    return ( count % 2 == 0 ) ? 1 : 0;
  }
  return 0;
}


