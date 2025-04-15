#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_even_length_binary_sequences_with_same_sum_of_first_and_second_half_bits_1 ( int n ) ;
int count_even_length_binary_sequences_with_same_sum_of_first_and_second_half_bits_1 ( int n ) {
  int nCr = 1, res = 1;
  for ( int r = 1;
  r <= n;
  r ++ ) {
    nCr = ( nCr * ( n + 1 - r ) ) / r;
    res += nCr * nCr;
  }
  return res;
}


