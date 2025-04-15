#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define OPL_EMU_REGISTERS_OPERATORS ( OPL_EMU_REGISTERS_CHANNELS * 2 )
#define OPL_EMU_REGISTERS_ALL_CHANNELS ( (1 << OPL_EMU_REGISTERS_CHANNELS) - 1 )
#define OPL_EMU_REGISTERS_RHYTHM_CHANNEL 0xff
#define OPL_EMU_REGISTERS_WAVEFORMS 8
#define OPL_EMU_REGISTERS_CHANNELS 18
#define OPL_EMU_REGISTERS_REGISTERS 0x200
#define OPL_EMU_REGISTERS_REG_MODE 0x04
#define OPL_EMU_REGISTERS_WAVEFORM_LENGTH 0x400

enum opl_emu_envelope_state
{
	OPL_EMU_EG_ATTACK = 1,
	OPL_EMU_EG_DECAY = 2,
	OPL_EMU_EG_SUSTAIN = 3,
	OPL_EMU_EG_RELEASE = 4,
	OPL_EMU_EG_STATES = 6
};
enum opl_emu_keyon_type
{
	OPL_EMU_KEYON_NORMAL = 0,
	OPL_EMU_KEYON_RHYTHM = 1,
	OPL_EMU_KEYON_CSM = 2
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
	struct opl_emu_fm_operator *m_op[4];
	struct opl_emu_registers* m_regs;
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
uint32_t opl_emu_bitfield(uint32_t value, int start, int length );
void opl_emu_fm_operator_keyonoff(struct opl_emu_fm_operator* fmop, uint32_t on, enum opl_emu_keyon_type type);
void opl_emu_fm_channel_keyonoff(struct opl_emu_fm_channel* fmch,uint32_t states, enum opl_emu_keyon_type type, uint32_t chnum);
int opl_emu_registers_write(struct opl_emu_registers* regs,uint16_t index, uint8_t data, uint32_t *channel, uint32_t *opmask);
void opl_write( opl_t* opl, int count, unsigned short* regs, unsigned char* data );
uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}
void opl_emu_fm_operator_keyonoff(struct opl_emu_fm_operator* fmop, uint32_t on, enum opl_emu_keyon_type type)
{
	fmop->m_keyon_live = (fmop->m_keyon_live & ~(1 << (int)(type))) | (opl_emu_bitfield(on, 0,1) << (int)(type));
}
void opl_emu_fm_channel_keyonoff(struct opl_emu_fm_channel* fmch,uint32_t states, enum opl_emu_keyon_type type, uint32_t chnum)
{
	for (uint32_t opnum = 0; opnum < sizeof( fmch->m_op ) / sizeof( *fmch->m_op ); opnum++)
		if (fmch->m_op[opnum] != NULL)
			opl_emu_fm_operator_keyonoff(fmch->m_op[opnum],opl_emu_bitfield(states, opnum,1), type);
}
int opl_emu_registers_write(struct opl_emu_registers* regs,uint16_t index, uint8_t data, uint32_t *channel, uint32_t *opmask)
{
	if (index == OPL_EMU_REGISTERS_REG_MODE && opl_emu_bitfield(data, 7,1) != 0)
		regs->m_regdata[index] |= 0x80;
	else
		regs->m_regdata[index] = data;
	if (index == 0xbd)
	{
		*channel = OPL_EMU_REGISTERS_RHYTHM_CHANNEL;
		*opmask = opl_emu_bitfield(data, 5,1) ? opl_emu_bitfield(data, 0, 5) : 0;
		return 1;
	}
	if ((index & 0xf0) == 0xb0)
	{
		*channel = index & 0x0f;
		if (*channel < 9)
		{
            *channel += 9 * opl_emu_bitfield(index, 8,1);
			*opmask = opl_emu_bitfield(data, 5,1) ? 15 : 0;
			return 1;
		}
	}
	return 0;
}
void opl_write( opl_t* opl, int count, unsigned short* regs, unsigned char* data ) {
    struct opl_emu_t* emu = &opl->opl_emu;
    for( int i = 0; i < count; ++i ) {
        uint16_t regnum = regs[ i ];
        uint8_t value = data[ i ];
	    if (regnum == OPL_EMU_REGISTERS_REG_MODE)
	    {
		    continue;;
	    }
	    emu->m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
	    uint32_t keyon_channel;
	    uint32_t keyon_opmask;
	    if (opl_emu_registers_write(&emu->m_regs,regnum, value, &keyon_channel, &keyon_opmask))
	    {
		    if (keyon_channel < OPL_EMU_REGISTERS_CHANNELS)
		    {
			    opl_emu_fm_channel_keyonoff(&emu->m_channel[keyon_channel],keyon_opmask, OPL_EMU_KEYON_NORMAL, keyon_channel);
		    }
		    else if (OPL_EMU_REGISTERS_CHANNELS >= 9 && keyon_channel == OPL_EMU_REGISTERS_RHYTHM_CHANNEL)
		    {
			    opl_emu_fm_channel_keyonoff(&emu->m_channel[6],opl_emu_bitfield(keyon_opmask, 4,1) ? 3 : 0, OPL_EMU_KEYON_RHYTHM, 6);
			    opl_emu_fm_channel_keyonoff(&emu->m_channel[7],opl_emu_bitfield(keyon_opmask, 0,1) | (opl_emu_bitfield(keyon_opmask, 3,1) << 1), OPL_EMU_KEYON_RHYTHM, 7);
			    opl_emu_fm_channel_keyonoff(&emu->m_channel[8],opl_emu_bitfield(keyon_opmask, 2,1) | (opl_emu_bitfield(keyon_opmask, 1,1) << 1), OPL_EMU_KEYON_RHYTHM, 8);
		    }
	    }
    }
}