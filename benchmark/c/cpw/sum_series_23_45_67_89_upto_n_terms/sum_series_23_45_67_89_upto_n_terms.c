

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



float sum_series_23_45_67_89_upto_n_terms ( int n ) ;
float sum_series_23_45_67_89_upto_n_terms ( int n ) {
  int i = 1;
  double res = 0.0;
  bool sign = 1;
  while ( n > 0 ) {
    n --;
    if ( sign ) {
      sign = ! sign;
      res = res + ( double ) ++ i / ++ i;
    }
    else {
      sign = ! sign;
      res = res - ( double ) ++ i / ++ i;
    }
  }
  return res;
}


