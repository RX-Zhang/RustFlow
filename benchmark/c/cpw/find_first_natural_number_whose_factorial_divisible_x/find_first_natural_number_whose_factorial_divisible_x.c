

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int find_first_natural_number_whose_factorial_divisible_x ( int x ) ;
int find_first_natural_number_whose_factorial_divisible_x ( int x ) {
  int i = 1;
  int fact = 1;
  for ( i = 1;
  i < x;
  i ++ ) {
    fact = fact * i;
    if ( fact % x == 0 ) break;
  }
  return i;
}


