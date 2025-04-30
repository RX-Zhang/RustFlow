

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



long long multiply_large_integers_under_large_modulo ( long long a, long long b, long long mod ) ;
long long multiply_large_integers_under_large_modulo ( long long a, long long b, long long mod ) {
  long long res = 0;
  a %= mod;
  while ( b ) {
    if ( b & 1 ) res = ( res + a ) % mod;
    a = ( 2 * a ) % mod;
    b >>= 1;
  }
  return res;
}


