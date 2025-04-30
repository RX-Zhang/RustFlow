

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int fibonacci_modulo_p ( int p ) ;
int fibonacci_modulo_p ( int p ) {
  int first = 1, second = 1, number = 2, next = 1;
  while ( next ) {
    next = ( first + second ) % p;
    first = second;
    second = next;
    number ++;
  }
  return number;
}


