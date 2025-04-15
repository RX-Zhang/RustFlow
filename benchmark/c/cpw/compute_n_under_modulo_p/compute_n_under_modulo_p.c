#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int compute_n_under_modulo_p ( int n, int p ) ;
int compute_n_under_modulo_p ( int n, int p ) {
  if ( n >= p ) return 0;
  int result = 1;
  for ( int i = 1;i <= n;i ++ )
  result = ( result * i ) % p;
  return result;
}


