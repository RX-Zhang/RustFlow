
use std::num::Wrapping;

fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level = Wrapping(!(*regbyte as i32));
    level &= Wrapping(0x3f);
    level = Wrapping((level.0 * volume) / 127);
    
    if level.0 > 0x3f {
        level = Wrapping(0x3f);
    }
    if level.0 < 0 {
        level = Wrapping(0);
    }
    
    level = !level;
    level &= Wrapping(0x3f);
    
    *regbyte &= 0xC0;
    *regbyte |= level.0 as u8;
}
