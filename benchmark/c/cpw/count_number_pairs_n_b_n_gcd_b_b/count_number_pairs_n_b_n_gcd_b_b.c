#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_number_pairs_n_b_n_gcd_b_b ( int n ) ;
int count_number_pairs_n_b_n_gcd_b_b ( int n ) {
  int k = n;
  int imin = 1;
  int ans = 0;
  while ( imin <= n ) {
    int imax = n / k;
    ans += k * ( imax - imin + 1 );
    imin = imax + 1;
    k = n / imin;
  }
  return ans;
}


