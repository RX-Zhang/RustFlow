use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Debug, Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

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
    eg_rate: [u8; OplEmuEnvelopeState::States as usize],
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

fn opl_min(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shr(start as u32)) & ((1u32.wrapping_shl(length as u32)).wrapping_sub(1))
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    let index = (offset + extra_offset) as usize;
    opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, count as i32)
}

fn opl_emu_registers_op_lfo_am_enable(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x20, 7, 1, opoffs)
}

fn opl_emu_fm_operator_envelope_attenuation(fmop: &OplEmuFmOperator, am_offset: u32) -> u32 {
    // Promote `m_env_attenuation` to `u32` early to avoid type mismatches
    let mut result = fmop.m_env_attenuation as u32;

    // Apply the envelope shift
    result = result.wrapping_shr(fmop.m_cache.eg_shift as u32);

    // Add AM offset if LFO AM is enabled
    if opl_emu_registers_op_lfo_am_enable(&fmop.m_regs, fmop.m_opoffs) != 0 {
        result = result.wrapping_add(am_offset);
    }

    // Add total level from the cache
    result = result.wrapping_add(fmop.m_cache.total_level as u32);

    // Ensure the result does not exceed 0x3FF
    opl_min(result, 0x3FF)
}