

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int sum_pairwise_products_1 ( int n ) ;
int sum_pairwise_products_1 ( int n ) {
  long int multiTerms = n * ( n + 1 ) / 2;
  long int sum = multiTerms;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    multiTerms = multiTerms - ( i - 1 );
    sum = sum + multiTerms * i;
  }
  return sum;
}


