#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define OPL_EMU_REGISTERS_OPERATORS ( OPL_EMU_REGISTERS_CHANNELS * 2 )
#define OPL_EMU_REGISTERS_WAVEFORMS 8
#define OPL_EMU_REGISTERS_CHANNELS 18
#define OPL_EMU_REGISTERS_REGISTERS 0x200
#define OPL_EMU_REGISTERS_WAVEFORM_LENGTH 0x400

enum opl_emu_envelope_state
{
	OPL_EMU_EG_ATTACK = 1,
	OPL_EMU_EG_DECAY = 2,
	OPL_EMU_EG_SUSTAIN = 3,
	OPL_EMU_EG_RELEASE = 4,
	OPL_EMU_EG_STATES = 6
};
enum op2_flags_t {
  OP2_FIXEDPITCH = 1,
  OP2_UNUSED = 2,
  OP2_DOUBLEVOICE = 4,
};
typedef struct opl_t opl_t;

struct opl_emu_registers
{
	uint16_t m_lfo_am_counter;
	uint16_t m_lfo_pm_counter;
	uint32_t m_noise_lfsr;
	uint8_t m_lfo_am;
	uint8_t m_regdata[OPL_EMU_REGISTERS_REGISTERS];
	uint16_t m_waveform[OPL_EMU_REGISTERS_WAVEFORMS][OPL_EMU_REGISTERS_WAVEFORM_LENGTH];
};

struct opl_emu_opdata_cache
{
	uint32_t phase_step;
	uint32_t total_level;
	uint32_t block_freq;
	int32_t detune;
	uint32_t multiple;
	uint32_t eg_sustain;
	uint8_t eg_rate[OPL_EMU_EG_STATES];
	uint8_t eg_shift;
};
struct opl_emu_fm_operator
{
	uint32_t m_choffs;
	uint32_t m_opoffs;
	uint32_t m_phase;
	uint16_t m_env_attenuation;
	enum opl_emu_envelope_state m_env_state;
	uint8_t m_key_state;
	uint8_t m_keyon_live;
	struct opl_emu_opdata_cache m_cache;
	struct opl_emu_registers* m_regs;
};
struct opl_emu_fm_channel
{
	uint32_t m_choffs;
	int16_t m_feedback[2];
	int16_t m_feedback_in;
};
typedef struct opl_timbre_t {
  unsigned long modulator_E862, carrier_E862;
  unsigned char modulator_40, carrier_40;
  unsigned char feedconn;
  signed char finetune;
  unsigned char notenum;
  signed short noteoffset;
} opl_timbre_t;
struct opl_emu_t
{
	uint32_t m_env_counter;
	uint8_t m_status;
	uint8_t m_timer_running[2];
	uint32_t m_active_channels;
	uint32_t m_modified_channels;
	uint32_t m_prepare_count;
	struct opl_emu_registers m_regs;
	struct opl_emu_fm_channel m_channel[OPL_EMU_REGISTERS_CHANNELS];
	struct opl_emu_fm_operator m_operator[OPL_EMU_REGISTERS_OPERATORS];
};
struct voicealloc_t {
  unsigned short priority;
  signed short timbreid;
  signed char channel;
  signed char note;
  unsigned char voiceindex;
};
struct opl_t {
  signed char notes2voices[16][128][2];
  unsigned short channelpitch[16];
  unsigned short channelvol[16];
  struct voicealloc_t voices2notes[18];
  unsigned char channelprog[16];
  int opl3;
  struct opl_emu_t opl_emu;
  struct opl_timbre_t opl_gmtimbres[ 256 ];
  struct opl_timbre_t opl_gmtimbres_voice2[ 256 ];
  int is_op2;
  enum op2_flags_t op2_flags[ 256 ];
};
static void opl_load_op2_voice(opl_timbre_t* timbre, uint8_t const* buff) ;
int opl_loadbank_op2(opl_t* opl, void const* data, int size ) ;
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
int opl_loadbank_op2(opl_t* opl, void const* data, int size ) {
  if( size < 8 + 36 * 175 ) {
      return -3;
  }
  uint8_t const* buff = (uint8_t const*) data;
  int i;
  if ((buff[0] != '#') || (buff[1] != 'O') || (buff[2] != 'P') || (buff[3] != 'L') || (buff[4] != '_') || (buff[5] != 'I') || (buff[6] != 'I') || (buff[7] != '#')) {
    return(-3);
  }
  buff += 8;
  opl->is_op2 = 1;
  for (i = 0; i < 175; i++) {
    opl->op2_flags[i] = (enum op2_flags_t)( buff[0] | ((uint16_t)buff[1] << 8) );
    int finetune = buff[2];
    uint8_t fixednote = buff[3];
    buff += 4;
    opl_load_op2_voice(&opl->opl_gmtimbres[i], buff);
    opl->opl_gmtimbres[i].notenum = fixednote;
    buff += 16;
    opl_load_op2_voice(&opl->opl_gmtimbres_voice2[i], buff);
    opl->opl_gmtimbres_voice2[i].notenum = fixednote;
    opl->opl_gmtimbres_voice2[i].finetune += finetune - 128;
    buff += 16;
  }
  return(0);
}