#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int circle_lattice_points ( int r ) ;
int circle_lattice_points ( int r ) {
  if ( r <= 0 ) return 0;
  int result = 4;
  for ( int x = 1;
  x < r;
  x ++ ) {
    int ySquare = r * r - x * x;
    int y = sqrt ( ySquare );
    if ( y * y == ySquare ) result += 4;
  }
  return result;
}


