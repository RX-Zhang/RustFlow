#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

static void calc_vol(unsigned char *regbyte, int volume) ;
static void calc_vol(unsigned char *regbyte, int volume) {
  int level;
  level = ~(*regbyte);
  level &= 0x3f;
  level = (level * volume) / 127;
  if (level > 0x3f) level = 0x3f;
  if (level < 0) level = 0;
  level = ~level;
  level &= 0x3f;
  *regbyte &= 0xC0;
  *regbyte |= level;
}