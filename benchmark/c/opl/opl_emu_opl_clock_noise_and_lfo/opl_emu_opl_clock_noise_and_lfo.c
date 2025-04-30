#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

uint32_t opl_emu_bitfield(uint32_t value, int start, int length );

static int32_t opl_emu_opl_clock_noise_and_lfo(uint32_t *noise_lfsr, uint16_t *lfo_am_counter, uint16_t *lfo_pm_counter, uint8_t *lfo_am, uint32_t am_depth, uint32_t pm_depth);
uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
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