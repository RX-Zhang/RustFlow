

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int friends_pairing_problem_2 ( int n ) ;
int friends_pairing_problem_2 ( int n ) {
  int a = 1, b = 2, c = 0;
  if ( n <= 2 ) {
    return n;
  }
  for ( int i = 3;
  i <= n;
  i ++ ) {
    c = b + ( i - 1 ) * a;
    a = b;
    b = c;
  }
  return c;
}


