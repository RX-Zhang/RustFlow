use std::cmp;

#[derive(Clone, Copy)]
pub enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

pub struct OplTimbre {
    modulator_e862: u32,
    carrier_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

pub struct Opl {
    notes2voices: [[[i8; 2]; 128]; 16],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [VoiceAlloc; 18],
    channelprog: [u8; 16],
    opl3: i32,
    opl_emu: OplEmu,
    opl_gmtimbres: Box<[OplTimbre; 256]>,
    opl_gmtimbres_voice2: Box<[OplTimbre; 256]>,
    is_op2: bool,
    op2_flags: [Op2Flags; 256],
}

pub struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

pub struct OplEmu {
    // Simplified for brevity
}

fn getinstrument(opl: &Opl, channel: i32, note: i32) -> i32 {
    if note < 0 || note > 127 || channel > 15 {
        return -1;
    }
    
    if channel == 9 {
        if opl.is_op2 {
            return 128 + cmp::max(0, note - 35);
        } else {
            return 128 | note;
        }
    }
    
    opl.channelprog[channel as usize] as i32
}
