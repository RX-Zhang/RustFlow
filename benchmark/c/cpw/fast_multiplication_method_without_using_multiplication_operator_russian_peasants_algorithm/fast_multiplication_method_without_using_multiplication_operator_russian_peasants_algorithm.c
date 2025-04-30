

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int fast_multiplication_method_without_using_multiplication_operator_russian_peasants_algorithm ( unsigned int a, unsigned int b ) ;
unsigned int fast_multiplication_method_without_using_multiplication_operator_russian_peasants_algorithm ( unsigned int a, unsigned int b ) {
  int res = 0;
  while ( b > 0 ) {
    if ( b & 1 ) res = res + a;
    a = a << 1;
    b = b >> 1;
  }
  return res;
}


