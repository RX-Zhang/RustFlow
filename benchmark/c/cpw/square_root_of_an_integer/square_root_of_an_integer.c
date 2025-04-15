

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int square_root_of_an_integer ( int x ) ;
int square_root_of_an_integer ( int x ) {
  if ( x == 0 || x == 1 ) return x;
  int i = 1, result = 1;
  while ( result <= x ) {
    i ++;
    result = i * i;
  }
  return i - 1;
}


