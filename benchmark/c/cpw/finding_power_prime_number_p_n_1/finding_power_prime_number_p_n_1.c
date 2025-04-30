

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int finding_power_prime_number_p_n_1 ( int n, int p ) ;
int finding_power_prime_number_p_n_1 ( int n, int p ) {
  int ans = 0;
  int temp = p;
  while ( temp <= n ) {
    ans += n / temp;
    temp = temp * p;
  }
  return ans;
}


