fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level = !(*regbyte as i32) & 0x3f;
    level = level.wrapping_mul(volume).wrapping_div(127);
    level = level.clamp(0, 0x3f);
    level = !level & 0x3f;
    *regbyte &= 0xC0;
    *regbyte |= level as u8;
}
