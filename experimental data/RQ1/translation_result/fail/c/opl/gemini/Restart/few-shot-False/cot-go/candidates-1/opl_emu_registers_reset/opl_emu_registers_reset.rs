
use std::num::Wrapping;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

fn opl_emu_registers_reset(regs: &mut OplEmuRegisters) {
    for i in 0..OPL_EMU_REGISTERS_REGISTERS {
        regs.m_regdata[i] = 0;
    }
}

fn new_opl_emu_registers() -> OplEmuRegisters {
    OplEmuRegisters {
        m_lfo_am_counter: 0,
        m_lfo_pm_counter: 0,
        m_noise_lfsr: 0,
        m_lfo_am: 0,
        m_regdata: Box::new([0; OPL_EMU_REGISTERS_REGISTERS]),
        m_waveform: Box::new([[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]),
    }
}
