

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int write_a_c_program_to_find_the_parity_of_an_unsigned_integer ( unsigned int n ) ;
int write_a_c_program_to_find_the_parity_of_an_unsigned_integer ( unsigned int n ) {
  bool parity = 0;
  while ( n ) {
    parity = ! parity;
    n = n & ( n - 1 );
  }
  return parity;
}


