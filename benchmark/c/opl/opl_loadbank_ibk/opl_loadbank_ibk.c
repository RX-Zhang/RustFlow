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
static int opl_loadbank_internal(opl_t* opl, char const* file, int offset) ;
int opl_loadbank_ibk(opl_t* opl, char const* file) ;
static int opl_loadbank_internal(opl_t* opl, char const* file, int offset) {
  opl->is_op2 = 0;
  unsigned char buff[16];
  int i;
  FILE* f = fopen( file, "rb" );
  if( !f ) return -1;
  fseek( f, 0, SEEK_END );
  if (ftell(f) != 3204) {
    fclose(f);
    return(-2);
  }
  fseek( f, 0, SEEK_SET);
  if ((fread(buff, 1, 4,f) != 4) || (buff[0] != 'I') || (buff[1] != 'B') || (buff[2] != 'K') || (buff[3] != 0x1A)) {
    fclose(f);
    return(-3);
  }
  for (i = offset; i < 128 + offset; i++) {
    if (fread(buff, 1, 16, f) != 16) {
      fclose(f);
      return(-4);
    }
    opl->opl_gmtimbres[i].modulator_E862 = buff[8];
    opl->opl_gmtimbres[i].modulator_E862 <<= 8;
    opl->opl_gmtimbres[i].modulator_E862 |= buff[6];
    opl->opl_gmtimbres[i].modulator_E862 <<= 8;
    opl->opl_gmtimbres[i].modulator_E862 |= buff[4];
    opl->opl_gmtimbres[i].modulator_E862 <<= 8;
    opl->opl_gmtimbres[i].modulator_E862 |= buff[0];
    opl->opl_gmtimbres[i].carrier_E862 = buff[9];
    opl->opl_gmtimbres[i].carrier_E862 <<= 8;
    opl->opl_gmtimbres[i].carrier_E862 |= buff[7];
    opl->opl_gmtimbres[i].carrier_E862 <<= 8;
    opl->opl_gmtimbres[i].carrier_E862 |= buff[5];
    opl->opl_gmtimbres[i].carrier_E862 <<= 8;
    opl->opl_gmtimbres[i].carrier_E862 |= buff[1];
    opl->opl_gmtimbres[i].modulator_40 = buff[2];
    opl->opl_gmtimbres[i].carrier_40 = buff[3];
    opl->opl_gmtimbres[i].feedconn = buff[10];
    opl->opl_gmtimbres[i].finetune = buff[12];
    opl->opl_gmtimbres[i].notenum = 60;
    opl->opl_gmtimbres[i].noteoffset = 0;
  }
  fclose(f);
  return(0);
}
int opl_loadbank_ibk(opl_t* opl, char const* file) {
  char *instruments = NULL, *percussion = NULL;
  int i, res;
  instruments = strdup(file);
  if (instruments == NULL) return(-64);
  for (i = 0; instruments[i] != 0; i++) {
    if (instruments[i] == ',') {
      instruments[i] = 0;
      percussion = instruments + i + 1;
      break;
    }
  }
  res = opl_loadbank_internal(opl, instruments, 0);
  if ((res == 0) && (percussion != NULL)) {
    res = opl_loadbank_internal(opl, percussion, 128);
  }
  free(instruments);
  return(res);
}