

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int k_th_prime_factor_given_number ( int n, int k ) ;
int k_th_prime_factor_given_number ( int n, int k ) {
  while ( n % 2 == 0 ) {
    k --;
    n = n / 2;
    if ( k == 0 ) return 2;
  }
  for ( int i = 3;
  i <= sqrt ( n );
  i = i + 2 ) {
    while ( n % i == 0 ) {
      if ( k == 1 ) return i;
      k --;
      n = n / i;
    }
  }
  if ( n > 2 && k == 1 ) return n;
  return - 1;
}


