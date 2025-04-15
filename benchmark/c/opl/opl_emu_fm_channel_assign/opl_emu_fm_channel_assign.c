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
	// internal state
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
void opl_emu_fm_operator_set_choffs(struct opl_emu_fm_operator* fmop,uint32_t choffs) ;
void opl_emu_fm_channel_assign(struct opl_emu_fm_channel* fmch,uint32_t index, struct opl_emu_fm_operator *op);

void opl_emu_fm_operator_set_choffs(struct opl_emu_fm_operator* fmop,uint32_t choffs) { fmop->m_choffs = choffs; }
void opl_emu_fm_channel_assign(struct opl_emu_fm_channel* fmch,uint32_t index, struct opl_emu_fm_operator *op)
{
	fmch->m_op[index] = op;
	if (op != NULL)
		opl_emu_fm_operator_set_choffs(op, fmch->m_choffs);
}