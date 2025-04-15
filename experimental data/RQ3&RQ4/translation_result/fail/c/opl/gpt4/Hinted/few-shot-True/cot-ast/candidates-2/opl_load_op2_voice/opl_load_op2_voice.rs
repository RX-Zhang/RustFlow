use std::boxed::Box;

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

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    timbre.modulator_e862 = ((buff[3] as u32) << 24)
        | ((buff[2] as u32) << 16)
        | ((buff[1] as u32) << 8)
        | (buff[0] as u32);
    timbre.carrier_e862 = ((buff[10] as u32) << 24)
        | ((buff[9] as u32) << 16)
        | ((buff[8] as u32) << 8)
        | (buff[7] as u32);
    timbre.modulator_40 = (buff[5] & 0x3f) | (buff[4] & 0xc0);
    timbre.carrier_40 = (buff[12] & 0x3f) | (buff[11] & 0xc0);
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = (buff[14] as i16) | ((buff[15] as i16) << 8)
}
