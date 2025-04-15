
use std::mem::size_of;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

#[repr(C)]
struct OplEmuRegistersOperatorMapping {
    chan: [u32; OPL_EMU_REGISTERS_CHANNELS],
}

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shr(start as u32)) & ((1u32.wrapping_shl(length as u32)).wrapping_sub(1))
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset.wrapping_add(extra_offset)) as usize] as u32, start as i32, count as i32)
}

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    o1 as u32 | ((o2 as u32) << 8) | ((o3 as u32) << 16) | ((o4 as u32) << 24)
}

fn opl_emu_registers_fourop_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x104, 0, 6, 0)
}

fn opl_emu_registers_operator_map(regs: &OplEmuRegisters, dest: &mut OplEmuRegistersOperatorMapping) {
    let fourop = opl_emu_registers_fourop_enable(regs);

    dest.chan[0] = if opl_emu_bitfield(fourop, 0, 1) != 0 {
        opl_emu_registers_operator_list(0, 3, 6, 9)
    } else {
        opl_emu_registers_operator_list(0, 3, 0xff, 0xff)
    };

    dest.chan[1] = if opl_emu_bitfield(fourop, 1, 1) != 0 {
        opl_emu_registers_operator_list(1, 4, 7, 10)
    } else {
        opl_emu_registers_operator_list(1, 4, 0xff, 0xff)
    };

    dest.chan[2] = if opl_emu_bitfield(fourop, 2, 1) != 0 {
        opl_emu_registers_operator_list(2, 5, 8, 11)
    } else {
        opl_emu_registers_operator_list(2, 5, 0xff, 0xff)
    };

    dest.chan[3] = if opl_emu_bitfield(fourop, 0, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(6, 9, 0xff, 0xff)
    };
}
