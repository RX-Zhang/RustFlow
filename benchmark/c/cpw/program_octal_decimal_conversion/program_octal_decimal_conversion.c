

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int program_octal_decimal_conversion ( int n ) ;
int program_octal_decimal_conversion ( int n ) {
  int num = n;
  int dec_value = 0;
  int base = 1;
  int temp = num;
  while ( temp ) {
    int last_digit = temp % 10;
    temp = temp / 10;
    dec_value += last_digit * base;
    base = base * 8;
  }
  return dec_value;
}


