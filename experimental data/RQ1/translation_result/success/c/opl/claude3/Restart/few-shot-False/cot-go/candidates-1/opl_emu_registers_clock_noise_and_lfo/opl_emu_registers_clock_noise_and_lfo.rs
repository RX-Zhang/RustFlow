
use std::mem::size_of;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset + extra_offset) as usize] as u32, start, count)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    *noise_lfsr = noise_lfsr.wrapping_shl(1);
    *noise_lfsr |= opl_emu_bitfield(*noise_lfsr, 23, 1) ^ opl_emu_bitfield(*noise_lfsr, 9, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 8, 1) ^ opl_emu_bitfield(*noise_lfsr, 1, 1);
    
    let am_counter = *lfo_am_counter as u32;
    *lfo_am_counter = if am_counter >= 210 * 64 - 1 { 0 } else { lfo_am_counter.wrapping_add(1) };

    let shift = 9 - 2 * am_depth;
    *lfo_am = ((if am_counter < 105 * 64 { am_counter } else { 210 * 64 + 63 - am_counter }) >> shift) as u8;

    let pm_counter = *lfo_pm_counter;
    *lfo_pm_counter = lfo_pm_counter.wrapping_add(1);

    let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    pm_scale[opl_emu_bitfield(pm_counter as u32, 10, 3) as usize] as i32 >> (pm_depth ^ 1)
}

fn opl_emu_registers_lfo_pm_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 6, 1, 0)
}

fn opl_emu_registers_lfo_am_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 7, 1, 0)
}

fn opl_emu_registers_clock_noise_and_lfo(regs: &mut OplEmuRegisters) -> i32 {
    let am_depth = opl_emu_registers_lfo_am_depth(regs);
    let pm_depth = opl_emu_registers_lfo_pm_depth(regs);
    
    opl_emu_opl_clock_noise_and_lfo(
        &mut regs.m_noise_lfsr,
        &mut regs.m_lfo_am_counter,
        &mut regs.m_lfo_pm_counter,
        &mut regs.m_lfo_am,
        am_depth,
        pm_depth
    )
}
