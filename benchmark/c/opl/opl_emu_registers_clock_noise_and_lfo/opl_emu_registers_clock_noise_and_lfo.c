#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define OPL_EMU_REGISTERS_WAVEFORMS 8
#define OPL_EMU_REGISTERS_REGISTERS 0x200
#define OPL_EMU_REGISTERS_WAVEFORM_LENGTH 0x400

struct opl_emu_registers
{
	uint16_t m_lfo_am_counter;
	uint16_t m_lfo_pm_counter;
	uint32_t m_noise_lfsr;
	uint8_t m_lfo_am;
	uint8_t m_regdata[OPL_EMU_REGISTERS_REGISTERS];
	uint16_t m_waveform[OPL_EMU_REGISTERS_WAVEFORMS][OPL_EMU_REGISTERS_WAVEFORM_LENGTH];
};

uint32_t opl_emu_bitfield(uint32_t value, int start, int length );
uint32_t opl_emu_registers_byte(struct opl_emu_registers* regs,uint32_t offset, uint32_t start, uint32_t count, uint32_t extra_offset/* = 0*/);
static int32_t opl_emu_opl_clock_noise_and_lfo(uint32_t *noise_lfsr, uint16_t *lfo_am_counter, uint16_t *lfo_pm_counter, uint8_t *lfo_am, uint32_t am_depth, uint32_t pm_depth)
;
uint32_t opl_emu_registers_lfo_pm_depth(struct opl_emu_registers* regs);
uint32_t opl_emu_registers_lfo_am_depth(struct opl_emu_registers* regs);
int32_t opl_emu_registers_clock_noise_and_lfo(struct opl_emu_registers* regs)
;
uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}
uint32_t opl_emu_registers_byte(struct opl_emu_registers* regs,uint32_t offset, uint32_t start, uint32_t count, uint32_t extra_offset/* = 0*/) 
{
	return opl_emu_bitfield(regs->m_regdata[offset + extra_offset], start, count);
}
static int32_t opl_emu_opl_clock_noise_and_lfo(uint32_t *noise_lfsr, uint16_t *lfo_am_counter, uint16_t *lfo_pm_counter, uint8_t *lfo_am, uint32_t am_depth, uint32_t pm_depth)
{
	*noise_lfsr <<= 1;
	*noise_lfsr |= opl_emu_bitfield(*noise_lfsr, 23,1) ^ opl_emu_bitfield(*noise_lfsr, 9,1) ^ opl_emu_bitfield(*noise_lfsr, 8,1) ^ opl_emu_bitfield(*noise_lfsr, 1,1);
	uint32_t am_counter = *lfo_am_counter++;
	if (am_counter >= 210*64 - 1)
		*lfo_am_counter = 0;
	int shift = 9 - 2 * am_depth;
	*lfo_am = ((am_counter < 105*64) ? am_counter : (210*64+63 - am_counter)) >> shift;
	uint32_t pm_counter = *lfo_pm_counter++;
	static int8_t pm_scale[8] = { 8, 4, 0, -4, -8, -4, 0, 4 };
	return pm_scale[opl_emu_bitfield(pm_counter, 10, 3)] >> (pm_depth ^ 1);
}
uint32_t opl_emu_registers_lfo_pm_depth(struct opl_emu_registers* regs)                     { return opl_emu_registers_byte(regs,0xbd, 6, 1, 0); }
uint32_t opl_emu_registers_lfo_am_depth(struct opl_emu_registers* regs)                     { return opl_emu_registers_byte(regs,0xbd, 7, 1, 0); }
int32_t opl_emu_registers_clock_noise_and_lfo(struct opl_emu_registers* regs)
{
	return opl_emu_opl_clock_noise_and_lfo(&regs->m_noise_lfsr, &regs->m_lfo_am_counter, &regs->m_lfo_pm_counter, &regs->m_lfo_am, opl_emu_registers_lfo_am_depth(regs), opl_emu_registers_lfo_pm_depth(regs));
}