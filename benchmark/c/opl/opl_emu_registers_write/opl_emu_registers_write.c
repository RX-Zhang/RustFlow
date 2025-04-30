#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define OPL_EMU_REGISTERS_RHYTHM_CHANNEL 0xff
#define OPL_EMU_REGISTERS_WAVEFORMS 8
#define OPL_EMU_REGISTERS_REGISTERS 0x200
#define OPL_EMU_REGISTERS_REG_MODE 0x04
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
int opl_emu_registers_write(struct opl_emu_registers* regs,uint16_t index, uint8_t data, uint32_t *channel, uint32_t *opmask);
uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
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