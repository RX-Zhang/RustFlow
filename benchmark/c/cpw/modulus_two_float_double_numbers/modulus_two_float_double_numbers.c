

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



float modulus_two_float_double_numbers ( double a, double b ) ;
float modulus_two_float_double_numbers ( double a, double b ) {
  double mod;
  if ( a < 0 ) mod = - a;
  else mod = a;
  if ( b < 0 ) b = - b;
  while ( mod >= b ) mod = mod - b;
  if ( a < 0 ) return - mod;
  return mod;
}


