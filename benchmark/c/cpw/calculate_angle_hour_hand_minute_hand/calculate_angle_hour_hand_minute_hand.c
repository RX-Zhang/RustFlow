#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int calculate_angle_hour_hand_minute_hand ( double h, double m ) ;
int calculate_angle_hour_hand_minute_hand ( double h, double m ) {
  if ( h < 0 || m < 0 || h > 12 || m > 60 ) printf ( "Wrong input" );
  if ( h == 12 ) h = 0;
  if ( m == 60 ) m = 0;
  int hour_angle = 0.5 * ( h * 60 + m );
  int minute_angle = 6 * m;
  int angle = abs ( hour_angle - minute_angle );
  angle = (( 360 - angle) < angle) ? 360 - angle : angle;
  return angle;
}


