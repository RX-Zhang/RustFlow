

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int smallest_of_three_integers_without_comparison_operators ( int x, int y, int z ) ;
int smallest_of_three_integers_without_comparison_operators ( int x, int y, int z ) {
  int c = 0;
  while ( x && y && z ) {
    x --;
    y --;
    z --;
    c ++;
  }
  return c;
}


