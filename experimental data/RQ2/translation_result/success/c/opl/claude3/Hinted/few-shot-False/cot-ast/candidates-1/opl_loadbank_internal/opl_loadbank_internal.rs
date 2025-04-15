
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

struct opl_t {
    is_op2: i32,
    opl_gmtimbres: [opl_gmtimbre; 128],
}

struct opl_gmtimbre {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i8,
}

fn opl_loadbank_internal(mut opl: Box<opl_t>, file: &str, offset: i32) -> i32 {
    opl.is_op2 = 0;
    let mut buff = [0u8; 16];
    let mut i: usize;

    let mut f = File::open(file).unwrap();

    f.seek(SeekFrom::End(0)).unwrap();
    if f.stream_position().unwrap() != 3204 {
        return -2;
    }

    f.seek(SeekFrom::Start(0)).unwrap();

    f.read(&mut buff).unwrap();
    if buff[0] != b'I' || buff[1] != b'B' || buff[2] != b'K' || buff[3] != 0x1A {
        return -3;
    }

    for i in offset..offset + 128 {
        f.read(&mut buff).unwrap();

        opl.opl_gmtimbres[i as usize].modulator_E862 = u32::from(buff[8])
            .wrapping_add(u32::from(buff[6]).wrapping_shl(8))
            .wrapping_add(u32::from(buff[4]).wrapping_shl(16))
            .wrapping_add(u32::from(buff[0]).wrapping_shl(24));

        opl.opl_gmtimbres[i as usize].carrier_E862 = u32::from(buff[9])
            .wrapping_add(u32::from(buff[7]).wrapping_shl(8))
            .wrapping_add(u32::from(buff[5]).wrapping_shl(16))
            .wrapping_add(u32::from(buff[1]).wrapping_shl(24));

        opl.opl_gmtimbres[i as usize].modulator_40 = buff[2];
        opl.opl_gmtimbres[i as usize].carrier_40 = buff[3];

        opl.opl_gmtimbres[i as usize].feedconn = buff[10];
        opl.opl_gmtimbres[i as usize].finetune = buff[12] as i8;
        opl.opl_gmtimbres[i as usize].notenum = 60;
        opl.opl_gmtimbres[i as usize].noteoffset = 0;
    }

    0
}

