fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level = (!*regbyte) as i32;
    level &= 0x3f;
    level = (level.wrapping_mul(volume) / 127).wrapping_add(0);
    if level > 0x3f {
        level = 0x3f;
    }
    if level < 0 {
        level = 0;
    }
    level = !level;
    level &= 0x3f;
    *regbyte &= 0xC0;
    *regbyte |= level as u8;
}
