

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int zeckendorfs_theorem_non_neighbouring_fibonacci_representation ( int n ) ;
int zeckendorfs_theorem_non_neighbouring_fibonacci_representation ( int n ) {
  if ( n == 0 || n == 1 ) return n;
  int f1 = 0, f2 = 1, f3 = 1;
  while ( f3 <= n ) {
    f1 = f2;
    f2 = f3;
    f3 = f1 + f2;
  }
  return f2;
}


