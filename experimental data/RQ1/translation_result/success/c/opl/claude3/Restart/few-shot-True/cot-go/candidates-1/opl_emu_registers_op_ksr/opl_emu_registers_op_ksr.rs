
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;

struct OplEmuRegisters {
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = (1u32.wrapping_shl(length as u32)).wrapping_sub(1);
    (value.wrapping_shr(start as u32)) & mask
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: u32, count: u32, extra_offset: usize) -> u32 {
    let index = offset.wrapping_add(extra_offset) % OPL_EMU_REGISTERS_REGISTERS;
    opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, count as i32)
}

fn opl_emu_registers_op_ksr(regs: &OplEmuRegisters, opoffs: usize) -> u32 {
    opl_emu_registers_byte(regs, 0x20, 4, 1, opoffs)
}
