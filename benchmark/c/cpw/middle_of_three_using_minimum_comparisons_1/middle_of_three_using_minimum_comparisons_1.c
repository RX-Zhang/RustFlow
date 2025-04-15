

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int middle_of_three_using_minimum_comparisons_1 ( int a, int b, int c ) ;
int middle_of_three_using_minimum_comparisons_1 ( int a, int b, int c ) {
  if ( a > b ) {
    if ( b > c ) return b;
    else if ( a > c ) return c;
    else return a;
  }
  else {
    if ( a > c ) return a;
    else if ( b > c ) return c;
    else return b;
  }
}


