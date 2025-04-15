#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int composite_number ( int n ) ;
int composite_number ( int n ) {
  if ( n <= 1 ) return 0;
  if ( n <= 3 ) return 0;
  if ( n % 2 == 0 || n % 3 == 0 ) return 1;
  for ( int i = 5;i * i <= n;i = i + 6 ) {
     if ( n % i == 0 || n % ( i + 2 ) == 0 ) return 1;
  }
  return 0;
}


