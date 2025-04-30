

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n ( int n ) ;
int minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n ( int n ) {
  int sum = 0;
  while ( n > 0 ) {
    sum += ( n % 10 );
    n /= 10;
  }
  if ( sum == 1 ) return 10;
  return sum;
}


