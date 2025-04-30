#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

uint32_t opl_emu_bitfield(uint32_t value, int start, int length );
static uint32_t opl_emu_opl_compute_phase_step(uint32_t block_freq, uint32_t multiple, int32_t lfo_raw_pm);

uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}
static uint32_t opl_emu_opl_compute_phase_step(uint32_t block_freq, uint32_t multiple, int32_t lfo_raw_pm)
{
	uint32_t fnum = opl_emu_bitfield(block_freq, 0, 10) << 2;
	fnum += (lfo_raw_pm * opl_emu_bitfield(block_freq, 7, 3)) >> 1;
	fnum &= 0xfff;
	uint32_t block = opl_emu_bitfield(block_freq, 10, 3);
	uint32_t phase_step = (fnum << block) >> 2;
	return (phase_step * multiple) >> 1;
}