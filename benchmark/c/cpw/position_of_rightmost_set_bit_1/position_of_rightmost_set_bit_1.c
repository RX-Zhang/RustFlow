

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int position_of_rightmost_set_bit_1 ( int n ) ;
int position_of_rightmost_set_bit_1 ( int n ) {
  int position = 1;
  int m = 1;
  while ( ! ( n & m ) ) {
    m = m << 1;
    position ++;
  }
  return position;
}


