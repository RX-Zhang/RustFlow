use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(PartialEq, Clone, Copy)]
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

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_attenuation_increment(rate: u32, index: u32) -> u32 {
    const S_INCREMENT_TABLE: [u32; 64] = [
        0x00000000, 0x00000000, 0x10101010, 0x10101010, // 0-3    (0x00-0x03)
        0x10101010, 0x10101010, 0x11101110, 0x11101110, // 4-7    (0x04-0x07)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 8-11   (0x08-0x0B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 12-15  (0x0C-0x0F)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 16-19  (0x10-0x13)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 20-23  (0x14-0x17)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 24-27  (0x18-0x1B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 28-31  (0x1C-0x1F)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 32-35  (0x20-0x23)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 36-39  (0x24-0x27)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 40-43  (0x28-0x2B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110, // 44-47  (0x2C-0x2F)
        0x11111111, 0x21112111, 0x21212121, 0x22212221, // 48-51  (0x30-0x33)
        0x22222222, 0x42224222, 0x42424242, 0x44424442, // 52-55  (0x34-0x37)
        0x44444444, 0x84448444, 0x84848484, 0x88848884, // 56-59  (0x38-0x3B)
        0x88888888, 0x88888888, 0x88888888, 0x88888888  // 60-63  (0x3C-0x3F)
    ];
    opl_emu_bitfield(S_INCREMENT_TABLE[rate as usize], 4 * index as i32, 4)
}

fn opl_emu_fm_operator_clock_envelope(fmop: &mut OplEmuFmOperator, env_counter: u32) {
    if fmop.m_env_state == OplEmuEnvelopeState::Attack && fmop.m_env_attenuation == 0 {
        fmop.m_env_state = OplEmuEnvelopeState::Decay;
    }
    if fmop.m_env_state == OplEmuEnvelopeState::Decay && fmop.m_env_attenuation >= fmop.m_cache.eg_sustain as u16 {
        fmop.m_env_state = OplEmuEnvelopeState::Sustain;
    }
    let rate = fmop.m_cache.eg_rate[fmop.m_env_state as usize];
    let rate_shift = rate >> 2;
    let env_counter = env_counter << rate_shift;
    if opl_emu_bitfield(env_counter, 0, 11) != 0 {
        return;
    }
    let relevant_bits = opl_emu_bitfield(env_counter, if rate_shift <= 11 { 11 } else { rate_shift as i32 }, 3);
    let increment = opl_emu_attenuation_increment(rate as u32, relevant_bits);
    if fmop.m_env_state == OplEmuEnvelopeState::Attack {
        if rate < 62 {
            fmop.m_env_attenuation = fmop.m_env_attenuation.wrapping_add((!fmop.m_env_attenuation * increment as u16) >> 4);
        }
    } else {
        fmop.m_env_attenuation = fmop.m_env_attenuation.wrapping_add(increment as u16);
        if fmop.m_env_attenuation >= 0x400 {
            fmop.m_env_attenuation = 0x3ff;
        }
    }
}
