

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int find_minimum_number_divided_make_number_perfect_square ( int n ) ;
int find_minimum_number_divided_make_number_perfect_square ( int n ) {
  int count = 0, ans = 1;
  while ( n % 2 == 0 ) {
    count ++;
    n /= 2;
  }
  if ( count % 2 ) ans *= 2;
  for ( int i = 3;
  i <= sqrt ( n );
  i += 2 ) {
    count = 0;
    while ( n % i == 0 ) {
      count ++;
      n /= i;
    }
    if ( count % 2 ) ans *= i;
  }
  if ( n > 2 ) ans *= n;
  return ans;
}


