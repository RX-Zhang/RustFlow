

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int sum_of_all_elements_up_to_nth_row_in_a_pascals_triangle ( int n ) ;
int sum_of_all_elements_up_to_nth_row_in_a_pascals_triangle ( int n ) {
  long int sum = 0;
  for ( int row = 0;
  row < n;
  row ++ ) {
    sum = sum + ( 1 << row );
  }
  return sum;
}


