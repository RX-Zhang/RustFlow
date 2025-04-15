#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int check_number_is_perfect_square_using_additionsubtraction ( int n ) ;
int check_number_is_perfect_square_using_additionsubtraction ( int n ) {
    for ( int sum = 0, i = 1;sum < n;i += 2 ) {
        sum += i;
        if ( sum == n ) return 1;
    }
    return 0;
}


