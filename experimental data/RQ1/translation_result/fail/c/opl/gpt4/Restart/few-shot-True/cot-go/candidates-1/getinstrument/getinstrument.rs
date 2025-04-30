
use std::boxed::Box;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
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

#[derive(Clone, Copy)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
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
    eg_rate: [u8; 6],
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
    m_op: [Box<OplEmuFmOperator>; 4],
    m_regs: Box<OplEmuRegisters>,
}

struct OplTimbre {
    modulator_e862: u32,
    carrier_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

struct OplEmu {
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

struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

struct Opl {
    notes2voices: [[[i8; 2]; 128]; 16],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [VoiceAlloc; 18],
    channelprog: [u8; 16],
    opl3: i32,
    opl_emu: OplEmu,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: bool,
    op2_flags: [Op2Flags; 256],
}

fn getinstrument(opl: &Opl, channel: i32, note: i32) -> i32 {
    if note < 0 || note > 127 || channel > 15 {
        return -1;
    }
    if channel == 9 {
        if opl.is_op2 {
            return 128 + note - 35;
        } else {
            return 128 | note;
        }
    }
    opl.channelprog[channel as usize] as i32
}
