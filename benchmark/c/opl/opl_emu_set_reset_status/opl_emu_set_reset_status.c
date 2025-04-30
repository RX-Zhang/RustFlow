#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define OPL_EMU_REGISTERS_OPERATORS ( OPL_EMU_REGISTERS_CHANNELS * 2 )
#define OPL_EMU_REGISTERS_STATUS_BUSY 0
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
uint32_t opl_emu_bitfield(uint32_t value, int start, int length );
uint32_t opl_emu_registers_byte(struct opl_emu_registers* regs,uint32_t offset, uint32_t start, uint32_t count, uint32_t extra_offset/* = 0*/);
uint32_t opl_emu_registers_status_mask(struct opl_emu_registers* regs);
uint8_t opl_emu_set_reset_status(struct opl_emu_t* emu, uint8_t set, uint8_t reset);

uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}
uint32_t opl_emu_registers_byte(struct opl_emu_registers* regs,uint32_t offset, uint32_t start, uint32_t count, uint32_t extra_offset/* = 0*/) 
{
	return opl_emu_bitfield(regs->m_regdata[offset + extra_offset], start, count);
}
uint32_t opl_emu_registers_status_mask(struct opl_emu_registers* regs)                      { return opl_emu_registers_byte(regs,0x04, 0, 8, 0) & 0x78; }
uint8_t opl_emu_set_reset_status(struct opl_emu_t* emu, uint8_t set, uint8_t reset)
{
	emu->m_status = (emu->m_status | set) & ~(reset | OPL_EMU_REGISTERS_STATUS_BUSY);
	//m_intf.opl_emu_sync_check_interrupts();
	return emu->m_status & ~opl_emu_registers_status_mask(&emu->m_regs);
}