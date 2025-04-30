

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

pub fn opl_emu_registers_reset_lfo(regs: &mut OplEmuRegisters) {
    regs.m_lfo_am_counter = 0;
    regs.m_lfo_pm_counter = 0;
}

impl Default for OplEmuRegisters {
    fn default() -> Self {
        OplEmuRegisters {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
            m_noise_lfsr: 0,
            m_lfo_am: 0,
            m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
            m_waveform: [[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
        }
    }
}
