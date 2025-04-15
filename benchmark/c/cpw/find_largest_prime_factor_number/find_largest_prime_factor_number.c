

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



long long find_largest_prime_factor_number ( long long n ) ;
long long find_largest_prime_factor_number ( long long n ) {
  long long maxPrime = - 1;
  while ( n % 2 == 0 ) {
    maxPrime = 2;
    n >>= 1;
  }
  for ( int i = 3;
  i <= sqrt ( n );
  i += 2 ) {
    while ( n % i == 0 ) {
      maxPrime = i;
      n = n / i;
    }
  }
  if ( n > 2 ) maxPrime = n;
  return maxPrime;
}


