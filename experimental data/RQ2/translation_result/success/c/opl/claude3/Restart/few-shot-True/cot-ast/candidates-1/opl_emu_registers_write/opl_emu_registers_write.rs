use std::boxed::Box;

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

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shr(start as u32)) & ((1u32.wrapping_shl(length as u32)).wrapping_sub(1))
}

fn opl_emu_registers_write(
    regs: &mut OplEmuRegisters,
    index: u16,
    data: u8,
    channel: &mut u32,
    opmask: &mut u32,
) -> i32 {
    // Handle OPL_EMU_REGISTERS_REG_MODE case
    if index == OPL_EMU_REGISTERS_REG_MODE && (data >> 7) != 0 {
        regs.m_regdata[index as usize] |= 0x80;
        return 0;
    }
    regs.m_regdata[index as usize] = data;

    // Handle rhythm mode (index == 0xbd)
    if index == 0xbd {
        *channel = OPL_EMU_REGISTERS_RHYTHM_CHANNEL;
        *opmask = if (data >> 5) & 1 != 0 { data & 0x1F } else { 0 };
        return 1;
    }

    // Handle channel selection (index in range 0xb0 to 0xbf)
    if (index & 0xf0) == 0xb0 {
        let chan_low = index & 0x0f;
        let high_bit = ((index >> 8) & 1) as u16;
        let chan = chan_low + (9 * high_bit);

        if chan < 9 {
            *channel = chan as u32;
            *opmask = if (data >> 5) & 1 != 0 { 15 } else { 0 };
            return 1;
        }
    }
    0
}