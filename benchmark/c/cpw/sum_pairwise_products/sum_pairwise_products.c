

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int sum_pairwise_products ( int n ) ;
int sum_pairwise_products ( int n ) {
  long int sum = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) for ( int j = i;
  j <= n;
  j ++ ) sum = sum + i * j;
  return sum;
}


