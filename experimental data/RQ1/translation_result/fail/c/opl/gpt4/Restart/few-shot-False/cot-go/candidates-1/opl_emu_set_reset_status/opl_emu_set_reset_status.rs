

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_STATUS_BUSY: u8 = 0;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

const OPL_EMU_EG_STATES: usize = 6;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OPL_EMU_EG_STATES],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: Box<OplEmuRegisters>,
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shr(start as u32) & ((1u32.wrapping_shl(length as u32)).wrapping_sub(1))) as u32
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: u32, count: u32, extra_offset: usize) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset + extra_offset] as u32, start as i32, count as i32)
}

fn opl_emu_registers_status_mask(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x04, 0, 8, 0) & 0x78
}

fn opl_emu_set_reset_status(emu: &mut OplEmuT, set: u8, reset: u8) -> u8 {
    emu.m_status = (emu.m_status | set) & !(reset | OPL_EMU_REGISTERS_STATUS_BUSY);
    emu.m_status & !opl_emu_registers_status_mask(&emu.m_regs) as u8
}
