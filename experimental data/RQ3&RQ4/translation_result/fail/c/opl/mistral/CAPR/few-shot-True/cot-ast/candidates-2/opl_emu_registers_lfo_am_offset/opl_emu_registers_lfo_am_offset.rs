struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

fn opl_emu_registers_lfo_am_offset(regs: &OplEmuRegisters, _choffs: u32) -> u32 {
    regs.m_lfo_am as u32
}
