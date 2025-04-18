#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define OPL_EMU_REGISTERS_WAVEFORMS 8
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
uint32_t opl_emu_bitfield(uint32_t value, int start, int length );
uint32_t opl_emu_attenuation_increment(uint32_t rate, uint32_t index);
void opl_emu_fm_operator_clock_envelope(struct opl_emu_fm_operator* fmop, uint32_t env_counter);

uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}
uint32_t opl_emu_attenuation_increment(uint32_t rate, uint32_t index)
{
	static uint32_t const s_increment_table[64] =
	{
		0x00000000, 0x00000000, 0x10101010, 0x10101010,  // 0-3    (0x00-0x03)
		0x10101010, 0x10101010, 0x11101110, 0x11101110,  // 4-7    (0x04-0x07)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 8-11   (0x08-0x0B)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 12-15  (0x0C-0x0F)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 16-19  (0x10-0x13)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 20-23  (0x14-0x17)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 24-27  (0x18-0x1B)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 28-31  (0x1C-0x1F)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 32-35  (0x20-0x23)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 36-39  (0x24-0x27)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 40-43  (0x28-0x2B)
		0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 44-47  (0x2C-0x2F)
		0x11111111, 0x21112111, 0x21212121, 0x22212221,  // 48-51  (0x30-0x33)
		0x22222222, 0x42224222, 0x42424242, 0x44424442,  // 52-55  (0x34-0x37)
		0x44444444, 0x84448444, 0x84848484, 0x88848884,  // 56-59  (0x38-0x3B)
		0x88888888, 0x88888888, 0x88888888, 0x88888888   // 60-63  (0x3C-0x3F)
	};
	return opl_emu_bitfield(s_increment_table[rate], 4*index, 4);
}
void opl_emu_fm_operator_clock_envelope(struct opl_emu_fm_operator* fmop, uint32_t env_counter)
{

	if (fmop->m_env_state == OPL_EMU_EG_ATTACK && fmop->m_env_attenuation == 0)
		fmop->m_env_state = OPL_EMU_EG_DECAY;
	if (fmop->m_env_state == OPL_EMU_EG_DECAY && fmop->m_env_attenuation >= fmop->m_cache.eg_sustain)
		fmop->m_env_state = OPL_EMU_EG_SUSTAIN;
	uint32_t rate = fmop->m_cache.eg_rate[fmop->m_env_state];
	uint32_t rate_shift = rate >> 2;
	env_counter <<= rate_shift;
	if (opl_emu_bitfield(env_counter, 0, 11) != 0)
		return;
	uint32_t relevant_bits = opl_emu_bitfield(env_counter, (rate_shift <= 11) ? 11 : rate_shift, 3);
	uint32_t increment = opl_emu_attenuation_increment(rate, relevant_bits);
	if (fmop->m_env_state == OPL_EMU_EG_ATTACK)
	{
		if (rate < 62)
			fmop->m_env_attenuation += (~fmop->m_env_attenuation * increment) >> 4;
	}
	else
	{
        fmop->m_env_attenuation += increment;
		if (fmop->m_env_attenuation >= 0x400)
			fmop->m_env_attenuation = 0x3ff;
	}
}