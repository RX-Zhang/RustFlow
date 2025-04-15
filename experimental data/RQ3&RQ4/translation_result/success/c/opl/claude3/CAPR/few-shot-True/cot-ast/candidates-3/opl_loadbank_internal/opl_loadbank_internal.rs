use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn opl_loadbank_internal(opl: &mut Box<OplT>, file: &str, offset: i32) -> i32 {
    opl.is_op2 = 0;
    let mut buff = [0u8; 16];
    
    let mut f = match File::open(file) {
        Ok(file) => file,
        Err(_) => return -1,
    };
    
    if let Ok(metadata) = f.metadata() {
        if metadata.len() != 3204 {
            return -2;
        }
    } else {
        return -2;
    }
    
    f.seek(SeekFrom::Start(0)).unwrap();
    
    if f.read_exact(&mut buff[..4]).is_err() || 
       buff[0] != b'I' || buff[1] != b'B' || buff[2] != b'K' || buff[3] != 0x1A {
        return -3;
    }
    
    for i in offset..128 + offset {
        if f.read_exact(&mut buff).is_err() {
            return -4;
        }
        
        let i = i as usize;
        opl.opl_gmtimbres[i].modulator_E862 = u32::from_le_bytes([buff[0], buff[4], buff[6], buff[8]]);
        opl.opl_gmtimbres[i].carrier_E862 = u32::from_le_bytes([buff[1], buff[5], buff[7], buff[9]]);
        opl.opl_gmtimbres[i].modulator_40 = buff[2];
        opl.opl_gmtimbres[i].carrier_40 = buff[3];
        opl.opl_gmtimbres[i].feedconn = buff[10];
        opl.opl_gmtimbres[i].finetune = buff[12] as i8;
        opl.opl_gmtimbres[i].notenum = 60;
        opl.opl_gmtimbres[i].noteoffset = 0;
    }
    
    0
}

// Struct definitions (not part of the function, but necessary for context)
struct OplT {
    is_op2: i32,
    opl_gmtimbres: [OplTimbreT; 256],
}

struct OplTimbreT {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}
