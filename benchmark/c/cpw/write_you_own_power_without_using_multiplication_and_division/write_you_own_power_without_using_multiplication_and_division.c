

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int write_you_own_power_without_using_multiplication_and_division ( int a, int b ) ;
int write_you_own_power_without_using_multiplication_and_division ( int a, int b ) {
  if ( b == 0 ) return 1;
  int answer = a;
  int increment = a;
  int i, j;
  for ( i = 1;
  i < b;
  i ++ ) {
    for ( j = 1;
    j < a;
    j ++ ) {
      answer += increment;
    }
    increment = answer;
  }
  return answer;
}


