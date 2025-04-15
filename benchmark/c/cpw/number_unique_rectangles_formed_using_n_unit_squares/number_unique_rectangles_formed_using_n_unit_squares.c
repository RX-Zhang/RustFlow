

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int number_unique_rectangles_formed_using_n_unit_squares ( int n ) ;
int number_unique_rectangles_formed_using_n_unit_squares ( int n ) {
  int ans = 0;
  for ( int length = 1;
  length <= sqrt ( n );
  ++ length ) for ( int height = length;
  height * length <= n;
  ++ height ) ans ++;
  return ans;
}


