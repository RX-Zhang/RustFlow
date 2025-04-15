#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

typedef struct opl_timbre_t {
  unsigned long modulator_E862, carrier_E862;
  unsigned char modulator_40, carrier_40;
  unsigned char feedconn;
  signed char finetune;
  unsigned char notenum;
  signed short noteoffset;
} opl_timbre_t;

static void opl_load_op2_voice(opl_timbre_t* timbre, uint8_t const* buff);
static void opl_load_op2_voice(opl_timbre_t* timbre, uint8_t const* buff) {
  timbre->modulator_E862 = buff[3];
  timbre->modulator_E862 <<= 8;
  timbre->modulator_E862 |= buff[2];
  timbre->modulator_E862 <<= 8;
  timbre->modulator_E862 |= buff[1];
  timbre->modulator_E862 <<= 8;
  timbre->modulator_E862 |= buff[0];
  timbre->carrier_E862 = buff[10];
  timbre->carrier_E862 <<= 8;
  timbre->carrier_E862 |= buff[9];
  timbre->carrier_E862 <<= 8;
  timbre->carrier_E862 |= buff[8];
  timbre->carrier_E862 <<= 8;
  timbre->carrier_E862 |= buff[7];
  timbre->modulator_40 = ( buff[5] & 0x3f ) | ( buff[4] & 0xc0 );
  timbre->carrier_40 = ( buff[12] & 0x3f ) | ( buff[11] & 0xc0 );
  timbre->feedconn = buff[6];
  timbre->finetune = 0;
  timbre->noteoffset = (int16_t)(buff[14] | ((uint16_t)buff[15] << 8));
}