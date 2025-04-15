
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
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
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
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
    is_op2: i32,
    op2_flags: [Op2Flags; 256],
}

fn opl_loadbank_internal(opl: &mut Box<Opl>, file: &str, offset: i32) -> i32 {
    opl.is_op2 = 0;
    let mut buff = [0u8; 16];
    let mut f = match File::open(file) {
        Ok(file) => file,
        Err(_) => return -1,
    };

    match f.seek(SeekFrom::End(0)) {
        Ok(size) if size != 3204 => return -2,
        Err(_) => return -2,
        _ => (),
    }

    f.seek(SeekFrom::Start(0)).unwrap();

    if f.read_exact(&mut buff[..4]).is_err() || &buff[..4] != b"IBK\x1A" {
        return -3;
    }

    for i in offset..128 + offset {
        if f.read_exact(&mut buff).is_err() {
            return -4;
        }

        let i = i as usize;
        opl.opl_gmtimbres[i].modulator_e862 = u32::from_le_bytes([buff[0], buff[4], buff[6], buff[8]]);
        opl.opl_gmtimbres[i].carrier_e862 = u32::from_le_bytes([buff[1], buff[5], buff[7], buff[9]]);
        opl.opl_gmtimbres[i].modulator_40 = buff[2];
        opl.opl_gmtimbres[i].carrier_40 = buff[3];
        opl.opl_gmtimbres[i].feedconn = buff[10];
        opl.opl_gmtimbres[i].finetune = buff[12] as i8;
        opl.opl_gmtimbres[i].notenum = 60;
        opl.opl_gmtimbres[i].noteoffset = 0;
    }

    0
}
