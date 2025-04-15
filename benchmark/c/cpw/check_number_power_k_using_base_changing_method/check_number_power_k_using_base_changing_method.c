#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int check_number_power_k_using_base_changing_method ( unsigned int n, unsigned int k ) ;
int check_number_power_k_using_base_changing_method ( unsigned int n, unsigned int k ) {
  bool oneSeen = 0;
  while ( n > 0 ) {
    int digit = n % k;
    if ( digit > 1 ) return 0;
    if ( digit == 1 ) {
      if ( oneSeen ) return 0;
      oneSeen = 1;
    }
    n /= k;
  }
  return 1;
}


