#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

uint32_t opl_emu_bitfield(uint32_t value, int start, int length );

uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}