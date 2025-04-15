

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int minimum_rotations_unlock_circular_lock ( int input, int unlock_code ) ;
int minimum_rotations_unlock_circular_lock ( int input, int unlock_code ) {
  int rotation = 0;
  int input_digit, code_digit;
  while ( input || unlock_code ) {
    input_digit = input % 10;
    code_digit = unlock_code % 10;
    rotation += min ( abs ( input_digit - code_digit ), 10 - abs ( input_digit - code_digit ) );
    input /= 10;
    unlock_code /= 10;
  }
  return rotation;
}


