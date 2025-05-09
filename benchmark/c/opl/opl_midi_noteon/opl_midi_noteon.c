#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define OPL_EMU_REGISTERS_ALL_CHANNELS ( (1 << OPL_EMU_REGISTERS_CHANNELS) - 1 )
#define OPL_EMU_REGISTERS_RHYTHM_CHANNEL 0xff
#define OPL_EMU_REGISTERS_WAVEFORMS 8
#define OPL_EMU_REGISTERS_WAVEFORM_LENGTH 0x400
#define OPL_EMU_REGISTERS_CHANNELS 18
#define OPL_EMU_REGISTERS_REGISTERS 0x200
#define OPL_EMU_REGISTERS_REG_MODE 0x04
#define OPL_EMU_REGISTERS_OPERATORS ( OPL_EMU_REGISTERS_CHANNELS * 2 )
#define OP2_2NDVOICE_PRIORITY_PENALTY 0xFF

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
const unsigned short op2offsets[18] = {0x03,0x04,0x05,0x0b,0x0c,0x0d,0x13,0x14,0x15,0x103,0x104,0x105,0x10b,0x10c,0x10d,0x113,0x114,0x115};
const unsigned short freqtable[128] = {                          /* note # */
        345, 365, 387, 410, 435, 460, 488, 517, 547, 580, 615, 651,  /*  0 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 12 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 24 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 36 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 48 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 60 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 72 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 84 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651,  /* 96 */
        690, 731, 774, 820, 869, 921, 975, 517, 547, 580, 615, 651, /* 108 */
        690, 731, 774, 820, 869, 921, 975, 517};
const unsigned short pitchtable[256] = {                    /* pitch wheel */
         29193U,29219U,29246U,29272U,29299U,29325U,29351U,29378U,  /* -128 */
         29405U,29431U,29458U,29484U,29511U,29538U,29564U,29591U,  /* -120 */
         29618U,29644U,29671U,29698U,29725U,29752U,29778U,29805U,  /* -112 */
         29832U,29859U,29886U,29913U,29940U,29967U,29994U,30021U,  /* -104 */
         30048U,30076U,30103U,30130U,30157U,30184U,30212U,30239U,  /*  -96 */
         30266U,30293U,30321U,30348U,30376U,30403U,30430U,30458U,  /*  -88 */
         30485U,30513U,30541U,30568U,30596U,30623U,30651U,30679U,  /*  -80 */
         30706U,30734U,30762U,30790U,30817U,30845U,30873U,30901U,  /*  -72 */
         30929U,30957U,30985U,31013U,31041U,31069U,31097U,31125U,  /*  -64 */
         31153U,31181U,31209U,31237U,31266U,31294U,31322U,31350U,  /*  -56 */
         31379U,31407U,31435U,31464U,31492U,31521U,31549U,31578U,  /*  -48 */
         31606U,31635U,31663U,31692U,31720U,31749U,31778U,31806U,  /*  -40 */
         31835U,31864U,31893U,31921U,31950U,31979U,32008U,32037U,  /*  -32 */
         32066U,32095U,32124U,32153U,32182U,32211U,32240U,32269U,  /*  -24 */
         32298U,32327U,32357U,32386U,32415U,32444U,32474U,32503U,  /*  -16 */
         32532U,32562U,32591U,32620U,32650U,32679U,32709U,32738U,  /*   -8 */
         32768U,32798U,32827U,32857U,32887U,32916U,32946U,32976U,  /*    0 */
         33005U,33035U,33065U,33095U,33125U,33155U,33185U,33215U,  /*    8 */
         33245U,33275U,33305U,33335U,33365U,33395U,33425U,33455U,  /*   16 */
         33486U,33516U,33546U,33576U,33607U,33637U,33667U,33698U,  /*   24 */
         33728U,33759U,33789U,33820U,33850U,33881U,33911U,33942U,  /*   32 */
         33973U,34003U,34034U,34065U,34095U,34126U,34157U,34188U,  /*   40 */
         34219U,34250U,34281U,34312U,34343U,34374U,34405U,34436U,  /*   48 */
         34467U,34498U,34529U,34560U,34591U,34623U,34654U,34685U,  /*   56 */
         34716U,34748U,34779U,34811U,34842U,34874U,34905U,34937U,  /*   64 */
         34968U,35000U,35031U,35063U,35095U,35126U,35158U,35190U,  /*   72 */
         35221U,35253U,35285U,35317U,35349U,35381U,35413U,35445U,  /*   80 */
         35477U,35509U,35541U,35573U,35605U,35637U,35669U,35702U,  /*   88 */
         35734U,35766U,35798U,35831U,35863U,35895U,35928U,35960U,  /*   96 */
         35993U,36025U,36058U,36090U,36123U,36155U,36188U,36221U,  /*  104 */
         36254U,36286U,36319U,36352U,36385U,36417U,36450U,36483U,  /*  112 */
         36516U,36549U,36582U,36615U,36648U,36681U,36715U,36748U};
static int voicescount = 9;
const unsigned short op1offsets[18] = {0x00,0x01,0x02,0x08,0x09,0x0a,0x10,0x11,0x12,0x100,0x101,0x102,0x108,0x109,0x10a,0x110,0x111,0x112};
const unsigned char octavetable[128] = {                         /* note # */
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,                          /*  0 */
        0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,                          /* 12 */
        1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2,                          /* 24 */
        2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,                          /* 36 */
        3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4,                          /* 48 */
        4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5,                          /* 60 */
        5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6,                          /* 72 */
        6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7,                          /* 84 */
        7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8,                          /* 96 */
        8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9,                         /* 108 */
        9, 9, 9, 9, 9, 9, 9,10};
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
void opl_emu_write( struct opl_emu_t* emu, uint16_t regnum, uint8_t data);
void oplregwr( opl_t* opl, uint16_t reg, uint8_t data ) ;
static void calc_vol(unsigned char *regbyte, int volume) ;
void opl_noteoff(opl_t* opl, unsigned short voice) ;
void opl_noteon(opl_t* opl, unsigned short voice, unsigned int note, int pitch) ;
static void voicevolume(opl_t* opl, unsigned short voice, const opl_timbre_t* timbre, int volume) ;
void opl_loadinstrument(opl_t* opl, int voice, opl_timbre_t *timbre) ;
static int getinstrument(opl_t* opl, int channel, int note) ;
void opl_midi_noteoff_op2(opl_t* opl, int channel, int note, int vindex) ;
void opl_midi_noteon_op2(opl_t* opl, int channel, int note, int velocity, int vindex) ;
void opl_midi_noteon(opl_t* opl, int channel, int note, int velocity) ;
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
void opl_emu_write( struct opl_emu_t* emu, uint16_t regnum, uint8_t data)
{
	if (regnum == OPL_EMU_REGISTERS_REG_MODE)
	{
		return;
	}
	emu->m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
	uint32_t keyon_channel;
	uint32_t keyon_opmask;
	if (opl_emu_registers_write(&emu->m_regs,regnum, data, &keyon_channel, &keyon_opmask))
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
void oplregwr( opl_t* opl, uint16_t reg, uint8_t data ) {
    opl_emu_write( &opl->opl_emu, reg, data );
}
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
void opl_noteoff(opl_t* opl, unsigned short voice) {
  if (voice >= 9) {
    oplregwr(opl, 0x1B0 + voice - 9, 0);
  } else {
    oplregwr(opl, 0xB0 + voice, 0);
  }
}
void opl_noteon(opl_t* opl, unsigned short voice, unsigned int note, int pitch) {
  unsigned int freq = freqtable[note];
  unsigned int octave = octavetable[note];

  if (pitch != 0) {
    if (pitch > 127) {
      pitch = 127;
    } else if (pitch < -128) {
      pitch = -128;
    }
    freq = ((unsigned long)freq * pitchtable[pitch + 128]) >> 15;
    if (freq >= 1024) {
      freq >>= 1;
      octave++;
    }
  }
  if (octave > 7) octave = 7;
  if (voice >= 9) {
    voice -= 9;
    voice |= 0x100;
  }
  oplregwr(opl, 0xA0 + voice, freq & 0xff); /* set lowfreq */
  oplregwr(opl, 0xB0 + voice, (freq >> 8) | (octave << 2) | 32);
}
static void voicevolume(opl_t* opl, unsigned short voice, const opl_timbre_t* timbre, int volume) {
  unsigned char carrierval = timbre->carrier_40;
  if (volume == 0) {
    carrierval |= 0x3f;
  } else {
    calc_vol(&carrierval, volume);
  }
  oplregwr(opl, 0x40 + op2offsets[voice], carrierval);
}
void opl_loadinstrument(opl_t* opl, int voice, opl_timbre_t *timbre) {
  oplregwr(opl, 0x40 + op1offsets[voice], timbre->modulator_40);
  oplregwr(opl, 0x40 + op2offsets[voice], timbre->carrier_40 | 0x3f);
  oplregwr(opl, 0xE0 + op1offsets[voice], timbre->modulator_E862 >> 24);
  oplregwr(opl, 0xE0 + op2offsets[voice], timbre->carrier_E862 >> 24);
  oplregwr(opl, 0x80 + op1offsets[voice], (timbre->modulator_E862 >> 16) & 0xff);
  oplregwr(opl, 0x80 + op2offsets[voice], (timbre->carrier_E862 >> 16) & 0xff);
  oplregwr(opl, 0x60 + op1offsets[voice], (timbre->modulator_E862 >> 8) & 0xff);
  oplregwr(opl, 0x60 + op2offsets[voice], (timbre->carrier_E862 >> 8) & 0xff);
  oplregwr(opl, 0x20 + op1offsets[voice], timbre->modulator_E862 & 0xff);
  oplregwr(opl, 0x20 + op2offsets[voice], timbre->carrier_E862 & 0xff);
  if (voice >= 9) {
    voice -= 9;
    voice |= 0x100;
  }
  if (opl->opl3 != 0) {
    oplregwr(opl, 0xC0 + voice, timbre->feedconn | 0x30);
  } else {
    oplregwr(opl, 0xC0 + voice, timbre->feedconn);
  }

}
static int getinstrument(opl_t* opl, int channel, int note) {
  if ((note < 0) || (note > 127) || (channel > 15)) return(-1);
  if (channel == 9) {
    if (opl->is_op2)
      return 128 + note - 35;
    else
      return(128 | note);
  }
  return(opl->channelprog[channel]);
}
void opl_midi_noteoff_op2(opl_t* opl, int channel, int note, int vindex) {
  int voice = opl->notes2voices[channel][note][vindex];

  if (voice >= 0) {
    opl_noteoff(opl, voice);
    opl->voices2notes[voice].channel = -1;
    opl->voices2notes[voice].note = -1;
    opl->voices2notes[voice].priority = -1;
    opl->voices2notes[voice].voiceindex = 0xFF;
    opl->notes2voices[channel][note][vindex] = -1;
  }
}
void opl_midi_noteon_op2(opl_t* opl, int channel, int note, int velocity, int vindex) {
  if( velocity == 0 ) {
      opl_midi_noteoff_op2( opl, channel, note, vindex );
      return;
  }
  int x, voice = -1;
  int lowestpriority = 0xFFFF;
  int highestvoiceindex = -1;
  int lowestpriorityvoice = -1;
  int instrument;
  instrument = getinstrument(opl, channel, note);
  if (instrument < 0) return;
  if (vindex > 0 && (opl->op2_flags[instrument] & OP2_DOUBLEVOICE) == 0) return;
  
  opl_timbre_t* timbre = vindex == 0 ? &(opl->opl_gmtimbres[instrument]) : &(opl->opl_gmtimbres_voice2[instrument]);
  if (opl->notes2voices[channel][note][vindex] >= 0) {
    voice = opl->notes2voices[channel][note][vindex];
  } else {
    for (x = 0; x < voicescount; x++) {
      if (opl->voices2notes[x].channel < 0) {
        voice = x;
        if (opl->voices2notes[x].timbreid == instrument && opl->voices2notes[x].voiceindex == vindex) {
          break;
        }
      }
      if (opl->voices2notes[x].priority < lowestpriority) {
        if (opl->voices2notes[x].voiceindex >= vindex && opl->voices2notes[x].voiceindex >= highestvoiceindex) {
          lowestpriorityvoice = x;
          lowestpriority = opl->voices2notes[x].priority;
          highestvoiceindex = opl->voices2notes[x].voiceindex;
        }
      }
    }
    if (voice < 0) {
      if (lowestpriorityvoice < 0) {
        return;
      }
      voice = lowestpriorityvoice;
      opl_midi_noteoff_op2(opl, opl->voices2notes[voice].channel, opl->voices2notes[voice].note, opl->voices2notes[voice].voiceindex);
    }
  }
  if (opl->voices2notes[voice].timbreid != instrument) {
    opl->voices2notes[voice].timbreid = instrument;
    opl_loadinstrument(opl, voice, timbre);
  }
  opl->voices2notes[voice].channel = channel;
  opl->voices2notes[voice].note = note;
  opl->voices2notes[voice].priority = ((16 - channel) << 8) | 0xff;
  opl->voices2notes[voice].voiceindex = vindex;
  opl->notes2voices[channel][note][vindex] = voice;
  if (vindex != 0) {
    int reducedprio = (int)opl->voices2notes[voice].priority - OP2_2NDVOICE_PRIORITY_PENALTY;
    if (reducedprio < 0) reducedprio = 0;
    opl->voices2notes[voice].priority = (unsigned short)reducedprio;
  }
  voicevolume(opl, voice, timbre, velocity * opl->channelvol[channel] / 127);
  if (channel == 9) {
    opl_noteon(opl, voice, timbre->notenum + timbre->noteoffset, opl->channelpitch[channel] + timbre->finetune);
  } else {
    opl_noteon(opl, voice, note + timbre->noteoffset, opl->channelpitch[channel] + timbre->finetune);
  }
  for (x = 0; x < voicescount; x++) {
    if (opl->voices2notes[x].priority > 0) opl->voices2notes[x].priority -= 1;
  }
}
void opl_midi_noteon(opl_t* opl, int channel, int note, int velocity) {
  opl_midi_noteon_op2(opl, channel, note, velocity, 1);
  opl_midi_noteon_op2(opl, channel, note, velocity, 0);
}