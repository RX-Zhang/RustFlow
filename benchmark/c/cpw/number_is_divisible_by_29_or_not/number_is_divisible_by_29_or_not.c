

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int number_is_divisible_by_29_or_not ( long int n ) ;
int number_is_divisible_by_29_or_not ( long int n ) {
  while ( n / 100 ) {
    int last_digit = n % 10;
    n /= 10;
    n += last_digit * 3;
  }
  return ( n % 29 == 0 );
}


