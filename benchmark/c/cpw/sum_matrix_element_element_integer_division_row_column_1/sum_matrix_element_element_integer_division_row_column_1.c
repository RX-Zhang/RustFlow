

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int sum_matrix_element_element_integer_division_row_column_1 ( int n ) ;
int sum_matrix_element_element_integer_division_row_column_1 ( int n ) {
  int ans = 0, temp = 0, num;
  for ( int i = 1;
  i <= n && temp < n;
  i ++ ) {
    temp = i - 1;
    num = 1;
    while ( temp < n ) {
      if ( temp + i <= n ) ans += ( i * num );
      else ans += ( ( n - temp ) * num );
      temp += i;
      num ++;
    }
  }
  return ans;
}


