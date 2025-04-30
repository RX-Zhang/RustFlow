

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

struct OplEmuRegistersOperatorMapping {
    chan: [u32; OPL_EMU_REGISTERS_CHANNELS],
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value.wrapping_shr(start as u32) & ((1 << length) - 1)) as u32
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: usize,
    start: usize,
    count: usize,
    extra_offset: usize,
) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset + extra_offset] as u32, start, count)
}

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    o1 as u32 | (o2 as u32) << 8 | (o3 as u32) << 16 | (o4 as u32) << 24
}

fn opl_emu_registers_fourop_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x104, 0, 6, 0)
}

fn opl_emu_registers_operator_map(
    regs: &OplEmuRegisters,
    dest: &mut OplEmuRegistersOperatorMapping,
) {
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

    dest.chan[4] = if opl_emu_bitfield(fourop, 1, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(7, 10, 0xff, 0xff)
    };

    dest.chan[5] = if opl_emu_bitfield(fourop, 2, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(8, 11, 0xff, 0xff)
    };

    dest.chan[6] = opl_emu_registers_operator_list(12, 15, 0xff, 0xff);
    dest.chan[7] = opl_emu_registers_operator_list(13, 16, 0xff, 0xff);
    dest.chan[8] = opl_emu_registers_operator_list(14, 17, 0xff, 0xff);

    dest.chan[9] = if opl_emu_bitfield(fourop, 3, 1) != 0 {
        opl_emu_registers_operator_list(18, 21, 24, 27)
    } else {
        opl_emu_registers_operator_list(18, 21, 0xff, 0xff)
    };

    dest.chan[10] = if opl_emu_bitfield(fourop, 4, 1) != 0 {
        opl_emu_registers_operator_list(19, 22, 25, 28)
    } else {
        opl_emu_registers_operator_list(19, 22, 0xff, 0xff)
    };

    dest.chan[11] = if opl_emu_bitfield(fourop, 5, 1) != 0 {
        opl_emu_registers_operator_list(20, 23, 26, 29)
    } else {
        opl_emu_registers_operator_list(20, 23, 0xff, 0xff)
    };

    dest.chan[12] = if opl_emu_bitfield(fourop, 3, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(24, 27, 0xff, 0xff)
    };

    dest.chan[13] = if opl_emu_bitfield(fourop, 4, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(25, 28, 0xff, 0xff)
    };

    dest.chan[14] = if opl_emu_bitfield(fourop, 5, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(26, 29, 0xff, 0xff)
    };

    dest.chan[15] = opl_emu_registers_operator_list(30, 33, 0xff, 0xff);
    dest.chan[16] = opl_emu_registers_operator_list(31, 34, 0xff, 0xff);
    dest.chan[17] = opl_emu_registers_operator_list(32, 35, 0xff, 0xff);
}
