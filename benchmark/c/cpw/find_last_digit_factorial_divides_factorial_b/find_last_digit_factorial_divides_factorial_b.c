

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int find_last_digit_factorial_divides_factorial_b ( long int A, long int B ) ;
int find_last_digit_factorial_divides_factorial_b ( long int A, long int B ) {
  int variable = 1;
  if ( A == B ) return 1;
  else if ( ( B - A ) >= 5 ) return 0;
  else {
    for ( long int i = A + 1;
    i <= B;
    i ++ ) variable = ( variable * ( i % 10 ) ) % 10;
    return variable % 10;
  }
}


