
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

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> (start as u32 % 32)) & ((1u32 << (length as u32)) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (Wrapping(offset) + Wrapping(extra_offset)).0 as usize;
    if index >= OPL_EMU_REGISTERS_REGISTERS {
        0 // Return 0 for out-of-bounds access
    } else {
        opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, count as i32)
    }
}

fn opl_emu_registers_op_lfo_pm_enable(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x20, 6, 1, opoffs)
}
