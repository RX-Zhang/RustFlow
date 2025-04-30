#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

unsigned int double_factorial_1 ( unsigned int n ) ;
unsigned int double_factorial_1 ( unsigned int n ) {
  int res = 1;
  for ( int i = n;
  i >= 0;
  i = i - 2 ) {
    if ( i == 0 || i == 1 ) return res;
    else res *= i;
  }
}


