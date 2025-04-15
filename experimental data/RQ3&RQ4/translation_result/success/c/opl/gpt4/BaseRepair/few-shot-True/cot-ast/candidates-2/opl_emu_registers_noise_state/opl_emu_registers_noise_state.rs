struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

fn opl_emu_registers_noise_state(regs: &OplEmuRegisters) -> u32 {
    regs.m_noise_lfsr >> 23
}
