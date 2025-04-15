
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

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shr(start as u32)) & ((1u32.wrapping_shl(length as u32 % 32)).wrapping_sub(1))
}

fn opl_emu_registers_byte(regs: &Box<OplEmuRegisters>, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (offset.wrapping_add(extra_offset)) as usize;
    let value = if index < OPL_EMU_REGISTERS_REGISTERS {
        regs.m_regdata[index] as u32
    } else {
        0
    };
    opl_emu_bitfield(value, start as i32, count as i32)
}

fn opl_emu_registers_word(regs: &Box<OplEmuRegisters>, offset1: u32, start1: u32, count1: u32, offset2: u32, start2: u32, count2: u32, extra_offset: u32) -> u32 {
    (opl_emu_registers_byte(regs, offset1, start1, count1, extra_offset).wrapping_shl(count2 % 32)) | 
    opl_emu_registers_byte(regs, offset2, start2, count2, extra_offset)
}

fn opl_emu_registers_ch_block_freq(regs: &Box<OplEmuRegisters>, choffs: u32) -> u32 {
    opl_emu_registers_word(regs, 0xb0, 0, 5, 0xa0, 0, 8, choffs)
}
