

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int program_check_plus_perfect_number ( int x ) ;
int program_check_plus_perfect_number ( int x ) {
  int temp = x;
  int n = 0;
  while ( x != 0 ) {
    x /= 10;
    n ++;
  }
  x = temp;
  int sum = 0;
  while ( x != 0 ) {
    sum += pow ( x % 10, n );
    x /= 10;
  }
  return ( sum == temp );
}


