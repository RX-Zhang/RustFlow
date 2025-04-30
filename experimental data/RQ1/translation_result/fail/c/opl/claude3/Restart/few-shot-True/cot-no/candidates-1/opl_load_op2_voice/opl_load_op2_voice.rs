
use std::mem::MaybeUninit;

#[repr(C)]
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

pub fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8; 16]) {
    timbre.modulator_e862 = u32::from_le_bytes([buff[0], buff[1], buff[2], buff[3]]);
    timbre.carrier_e862 = u32::from_le_bytes([buff[7], buff[8], buff[9], buff[10]]);
    timbre.modulator_40 = buff[4];
    timbre.carrier_40 = buff[11];
    timbre.feedconn = buff[5];
    timbre.finetune = buff[6] as i8;
    timbre.notenum = buff[12];
    timbre.noteoffset = i16::from_le_bytes([buff[13], buff[14]]);
}
