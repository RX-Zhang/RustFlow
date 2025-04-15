

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int number_digits_product_two_numbers ( int a, int b ) ;
int number_digits_product_two_numbers ( int a, int b ) {
  int count = 0;
  int p = abs ( a * b );
  if ( p == 0 ) return 1;
  while ( p > 0 ) {
    count ++;
    p = p / 10;
  }
  return count;
}


