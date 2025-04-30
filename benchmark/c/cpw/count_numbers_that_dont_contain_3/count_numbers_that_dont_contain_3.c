#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int count_numbers_that_dont_contain_3 ( int n ) ;
int count_numbers_that_dont_contain_3 ( int n ) {
  if ( n < 3 ) return n;
  if ( n >= 3 && n < 10 ) return n - 1;
  int po = 1;
  while ( n / po > 9 ) po = po * 10;
  int msd = n / po;
  if ( msd != 3 ) return count_numbers_that_dont_contain_3 ( msd ) * count_numbers_that_dont_contain_3 ( po - 1 ) + count_numbers_that_dont_contain_3 ( msd ) + count_numbers_that_dont_contain_3 ( n % po );
  else return count_numbers_that_dont_contain_3 ( msd * po - 1 );
}


