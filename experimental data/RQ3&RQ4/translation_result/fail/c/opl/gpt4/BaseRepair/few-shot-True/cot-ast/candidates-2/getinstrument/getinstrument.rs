use std::boxed::Box;

#[derive(Copy, Clone)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

#[derive(Copy, Clone)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
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
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
    regs: Box<OplEmuRegisters>,
}

struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
    op: [Box<OplEmuFmOperator>; 4],
    regs: Box<OplEmuRegisters>,
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
    env_counter: u32,
    status: u8,
    timer_running: [u8; 2],
    active_channels: u32,
    modified_channels: u32,
    prepare_count: u32,
    regs: OplEmuRegisters,
    channel: [OplEmuFmChannel; 18],
    operator: [OplEmuFmOperator; 36],
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
    is_op2: i32,
    op2_flags: [Op2Flags; 256],
}

fn getinstrument(opl: &mut Opl, channel: i32, note: i32) -> i32 {
    if note < 0 || note > 127 || channel > 15 {
        return -1;
    }
    if channel == 9 {
        if opl.is_op2 != 0 {
            return 128 + note - 35
        } else {
            return 128 | note;
        }
    }
    opl.channelprog[channel as usize] as i32
}
