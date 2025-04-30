

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



long painting_fence_algorithm ( int n, int k ) ;
long painting_fence_algorithm ( int n, int k ) {
  long total = k;
  int mod = 1000000007;
  int same = 0, diff = k;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    same = diff;
    diff = total * ( k - 1 );
    diff = diff % mod;
    total = ( same + diff ) % mod;
  }
  return total;
}


