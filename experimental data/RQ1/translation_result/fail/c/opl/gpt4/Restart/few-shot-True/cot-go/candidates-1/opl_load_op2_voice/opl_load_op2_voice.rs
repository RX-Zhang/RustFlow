

#[derive(Default)]
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
    if buff.len() < 16 {
        return;
    }

    timbre.modulator_e862 = u32::from_le_bytes([buff[0], buff[1], buff[2], buff[3]]);
    timbre.carrier_e862 = u32::from_le_bytes([buff[7], buff[8], buff[9], buff[10]]);

    timbre.modulator_40 = (buff[5] & 0x3f) | (buff[4] & 0xc0);
    timbre.carrier_40 = (buff[12] & 0x3f) | (buff[11] & 0xc0);
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = i16::from_le_bytes([buff[14], buff[15]]);
}
