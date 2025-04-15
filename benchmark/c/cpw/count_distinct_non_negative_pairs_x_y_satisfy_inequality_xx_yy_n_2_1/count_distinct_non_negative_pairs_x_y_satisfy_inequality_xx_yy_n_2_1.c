#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2_1 ( int n ) ;
int count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2_1 ( int n ) {
  int x = 0, yCount, res = 0;
  for ( yCount = 0;
  yCount * yCount < n;
  yCount ++ );
  while ( yCount != 0 ) {
    res += yCount;
    x ++;
    while ( yCount != 0 && ( x * x + ( yCount - 1 ) * ( yCount - 1 ) >= n ) ) yCount --;
  }
  return res;
}


