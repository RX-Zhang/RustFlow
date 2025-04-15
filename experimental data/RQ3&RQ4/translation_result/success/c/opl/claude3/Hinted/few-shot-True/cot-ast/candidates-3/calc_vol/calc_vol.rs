fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level = !(*regbyte as i32) & 0x3f;
    level = (level.wrapping_mul(volume) / 127).clamp(0, 0x3f);
    level = !level & 0x3f;
    *regbyte = (*regbyte & 0xC0) | (level as u8);
}
