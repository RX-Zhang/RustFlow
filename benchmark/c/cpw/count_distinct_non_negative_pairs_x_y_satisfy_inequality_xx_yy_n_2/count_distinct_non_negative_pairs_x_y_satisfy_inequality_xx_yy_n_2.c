#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2 ( int n ) ;
int count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2 ( int n ) {
  int res = 0;
  for ( int x = 0;
  x * x < n;
  x ++ ) for ( int y = 0;
  x * x + y * y < n;
  y ++ ) res ++;
  return res;
}


