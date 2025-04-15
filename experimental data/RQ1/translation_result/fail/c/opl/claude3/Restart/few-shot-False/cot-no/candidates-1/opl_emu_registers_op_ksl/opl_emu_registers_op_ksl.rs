

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

pub struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    value.wrapping_shr(start as u32) & ((1u32.wrapping_shl(length as u32)).wrapping_sub(1))
}

pub fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset + extra_offset) as usize] as u32, start as i32, count as i32)
}

pub fn opl_emu_registers_op_ksl(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    let temp = opl_emu_registers_byte(regs, 0x40, 6, 2, opoffs);
    opl_emu_bitfield(temp, 1, 1) | (opl_emu_bitfield(temp, 0, 1).wrapping_shl(1))
}
