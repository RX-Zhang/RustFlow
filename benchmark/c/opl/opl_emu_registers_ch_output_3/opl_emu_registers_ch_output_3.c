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
uint32_t opl_emu_registers_newflag(struct opl_emu_registers* regs);
uint32_t opl_emu_registers_ch_output_3(struct opl_emu_registers* regs,uint32_t choffs);
uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}
uint32_t opl_emu_registers_byte(struct opl_emu_registers* regs,uint32_t offset, uint32_t start, uint32_t count, uint32_t extra_offset/* = 0*/) 
{
	return opl_emu_bitfield(regs->m_regdata[offset + extra_offset], start, count);
}
uint32_t opl_emu_registers_newflag(struct opl_emu_registers* regs)                          { return opl_emu_registers_byte(regs,0x105, 0, 1, 0); }
uint32_t opl_emu_registers_ch_output_3(struct opl_emu_registers* regs,uint32_t choffs)       { return opl_emu_registers_newflag(regs) ? opl_emu_registers_byte(regs,0xc0 + choffs, 7, 1, 0) : 0; }