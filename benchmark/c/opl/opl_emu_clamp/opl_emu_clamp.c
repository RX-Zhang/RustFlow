#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

int32_t opl_emu_clamp(int32_t value, int32_t minval, int32_t maxval);

int32_t opl_emu_clamp(int32_t value, int32_t minval, int32_t maxval)
{
	if (value < minval)
		return minval;
	if (value > maxval)
		return maxval;
	return value;
}