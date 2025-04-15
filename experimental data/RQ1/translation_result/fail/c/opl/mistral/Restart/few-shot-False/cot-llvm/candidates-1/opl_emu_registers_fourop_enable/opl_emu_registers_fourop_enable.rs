

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

pub struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let shifted = value.wrapping_shr(start as u32);
    shifted & ((1u32 << length).wrapping_sub(1))
}

pub fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (offset + extra_offset) as usize;
    opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, count as i32)
}

pub fn opl_emu_registers_fourop_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x104, 0, 6, 0)
}
