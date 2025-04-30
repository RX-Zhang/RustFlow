

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int sum_factors_number ( int n ) ;
int sum_factors_number ( int n ) {
  int result = 0;
  for ( int i = 2;
  i <= sqrt ( n );
  i ++ ) {
    if ( n % i == 0 ) {
      if ( i == ( n / i ) ) result += i;
      else result += ( i + n / i );
    }
  }
  return ( result + n + 1 );
}


