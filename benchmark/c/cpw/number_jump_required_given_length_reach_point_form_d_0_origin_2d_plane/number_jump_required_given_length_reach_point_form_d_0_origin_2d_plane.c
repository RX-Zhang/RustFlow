

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int number_jump_required_given_length_reach_point_form_d_0_origin_2d_plane ( int a, int b, int d ) ;
int number_jump_required_given_length_reach_point_form_d_0_origin_2d_plane ( int a, int b, int d ) {
  int temp = a;
  a = min ( a, b );
  b = max ( temp, b );
  if ( d >= b ) return ( d + b - 1 ) / b;
  if ( d == 0 ) return 0;
  if ( d == a ) return 1;
  return 2;
}


