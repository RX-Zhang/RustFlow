struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: Box<[[u16; 0x400]; 8]>,
}

fn opl_emu_registers_reset_lfo(regs: &mut OplEmuRegisters) {
    regs.m_lfo_am_counter = 0;
    regs.m_lfo_pm_counter = 0;
}
