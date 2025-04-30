

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1 << OPL_EMU_REGISTERS_CHANNELS) - 1;
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy)]
enum OplEmuKeyonType {
    Normal = 0,
    Rhythm = 1,
    Csm = 2,
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
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
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
    is_op2: i32,
    op2_flags: [Op2Flags; 256],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, key_type: OplEmuKeyonType) {
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << key_type as u8)) | ((opl_emu_bitfield(on, 0, 1) << key_type as u8) as u8);
}

fn opl_emu_fm_channel_keyonoff(fmch: &mut OplEmuFmChannel, states: u32, key_type: OplEmuKeyonType, chnum: u32) {
    for (opnum, op) in fmch.m_op.iter_mut().enumerate() {
        if let Some(operator) = op {
            opl_emu_fm_operator_keyonoff(operator, opl_emu_bitfield(states, opnum as i32, 1), key_type);
        }
    }
}

fn opl_emu_registers_write(regs: &mut OplEmuRegisters, index: u16, data: u8, channel: &mut u32, opmask: &mut u32) -> bool {
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
        return true;
    }

    if (index & 0xf0) == 0xb0 {
        *channel = (index & 0x0f) as u32;
        if *channel < 9 {
            *channel += 9 * opl_emu_bitfield(index as u32, 8, 1);
            *opmask = if opl_emu_bitfield(data as u32, 5, 1) != 0 { 15 } else { 0 };
            return true;
        }
    }

    false
}

fn opl_write(opl: &mut Opl, count: i32, regs: &[u16], data: &[u8]) {
    let emu = &mut opl.opl_emu;
    for i in 0..count {
        let regnum = regs[i as usize];
        let value = data[i as usize];
        if regnum == OPL_EMU_REGISTERS_REG_MODE {
            continue;
        }
        emu.m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
        let mut keyon_channel = 0;
        let mut keyon_opmask = 0;
        if opl_emu_registers_write(&mut emu.m_regs, regnum, value, &mut keyon_channel, &mut keyon_opmask) {
            if keyon_channel < OPL_EMU_REGISTERS_CHANNELS as u32 {
                opl_emu_fm_channel_keyonoff(&mut emu.m_channel[keyon_channel as usize], keyon_opmask, OplEmuKeyonType::Normal, keyon_channel);
            } else if OPL_EMU_REGISTERS_CHANNELS >= 9 && keyon_channel == OPL_EMU_REGISTERS_RHYTHM_CHANNEL {
                opl_emu_fm_channel_keyonoff(&mut emu.m_channel[6], if opl_emu_bitfield(keyon_opmask, 4, 1) != 0 { 3 } else { 0 }, OplEmuKeyonType::Rhythm, 6);
                opl_emu_fm_channel_keyonoff(&mut emu.m_channel[7], opl_emu_bitfield(keyon_opmask, 0, 1) | (opl_emu_bitfield(keyon_opmask, 3, 1) << 1), OplEmuKeyonType::Rhythm, 7);
                opl_emu_fm_channel_keyonoff(&mut emu.m_channel[8], opl_emu_bitfield(keyon_opmask, 2, 1) | (opl_emu_bitfield(keyon_opmask, 1, 1) << 1), OplEmuKeyonType::Rhythm, 8);
            }
        }
    }
}
