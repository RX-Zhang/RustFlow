#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_possible_paths_top_left_bottom_right_nxm_matrix_3 ( int m, int n ) ;
int count_possible_paths_top_left_bottom_right_nxm_matrix_3 ( int m, int n ) {
  int path = 1;
  for ( int i = n;
  i < ( m + n - 1 );
  i ++ ) {
    path *= i;
    path /= ( i - n + 1 );
  }
  return path;
}


