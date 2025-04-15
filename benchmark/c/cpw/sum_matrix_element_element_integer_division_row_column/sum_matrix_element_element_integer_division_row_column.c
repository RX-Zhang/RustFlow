

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int sum_matrix_element_element_integer_division_row_column ( int n ) ;
int sum_matrix_element_element_integer_division_row_column ( int n ) {
  int ans = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) for ( int j = 1;
  j <= n;
  j ++ ) ans += ( i / j );
  return ans;
}


