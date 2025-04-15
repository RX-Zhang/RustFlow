#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_possible_ways_to_construct_buildings ( int N ) ;
int count_possible_ways_to_construct_buildings ( int N ) {
  if ( N == 1 ) return 4;
  int countB = 1, countS = 1, prev_countB, prev_countS;
  for ( int i = 2;
  i <= N;
  i ++ ) {
    prev_countB = countB;
    prev_countS = countS;
    countS = prev_countB + prev_countS;
    countB = prev_countS;
  }
  int result = countS + countB;
  return ( result * result );
}


