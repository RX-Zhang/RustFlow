
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;

struct OplEmuRegisters {
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value.wrapping_shr(start % 32)) & ((1u32.wrapping_shl(length % 32)).wrapping_sub(1))
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: u32, count: u32, extra_offset: usize) -> u32 {
    let index = (offset + extra_offset) % OPL_EMU_REGISTERS_REGISTERS;
    opl_emu_bitfield(regs.m_regdata[index] as u32, start, count)
}

fn opl_emu_registers_ch_feedback(regs: &OplEmuRegisters, choffs: usize) -> u32 {
    opl_emu_registers_byte(regs, 0xc0, 1, 3, choffs % OPL_EMU_REGISTERS_REGISTERS)
}
