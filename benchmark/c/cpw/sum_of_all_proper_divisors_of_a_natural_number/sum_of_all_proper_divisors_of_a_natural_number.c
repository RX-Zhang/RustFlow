

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int sum_of_all_proper_divisors_of_a_natural_number ( int num ) ;
int sum_of_all_proper_divisors_of_a_natural_number ( int num ) {
  int result = 0;
  for ( int i = 2;
  i <= sqrt ( num );
  i ++ ) {
    if ( num % i == 0 ) {
      if ( i == ( num / i ) ) result += i;
      else result += ( i + num / i );
    }
  }
  return ( result + 1 );
}


