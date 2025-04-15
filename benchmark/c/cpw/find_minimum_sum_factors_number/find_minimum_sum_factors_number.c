

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int find_minimum_sum_factors_number ( int num ) ;
int find_minimum_sum_factors_number ( int num ) {
  int sum = 0;
  for ( int i = 2;
  i * i <= num;
  i ++ ) {
    while ( num % i == 0 ) {
      sum += i;
      num /= i;
    }
  }
  sum += num;
  return sum;
}


