
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
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
    (value.wrapping_shr(start % 32)) & ((1u32.wrapping_shl(length % 32)).wrapping_sub(1))
}

fn opl_emu_registers_write(regs: &mut OplEmuRegisters, index: u16, data: u8, channel: &mut u32, opmask: &mut u32) -> i32 {
    if index as usize >= OPL_EMU_REGISTERS_REGISTERS {
        return 0;
    }

    if index == OPL_EMU_REGISTERS_REG_MODE && opl_emu_bitfield(data as u32, 7, 1) != 0 {
        regs.m_regdata[index as usize] |= 0x80;
    } else {
        regs.m_regdata[index as usize] = data;
    }

    if index == 0xbd {
        *channel = OPL_EMU_REGISTERS_RHYTHM_CHANNEL;
        *opmask = if opl_emu_bitfield(data as u32, 5, 1) != 0 {
            opl_emu_bitfield(data as u32, 0, 5)
        } else {
            0
        };
        return 1;
    }

    if (index & 0xf0) == 0xb0 {
        *channel = (index & 0x0f) as u32;
        if *channel < 9 {
            *channel = channel.wrapping_add(9 * opl_emu_bitfield(index as u32, 8, 1));
            *opmask = if opl_emu_bitfield(data as u32, 5, 1) != 0 { 15 } else { 0 };
            return 1;
        }
    }

    0
}
