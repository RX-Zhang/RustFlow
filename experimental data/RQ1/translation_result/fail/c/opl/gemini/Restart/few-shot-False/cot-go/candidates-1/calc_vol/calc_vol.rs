
use std::cmp;

fn calc_vol(regbyte: &mut u8, volume: i32) {
    let mut level = !(*regbyte as i32) & 0x3f;
    level = (level * volume).wrapping_div(127);
    level = cmp::min(cmp::max(level, 0), 0x3f);
    level = !level & 0x3f;
    *regbyte = (*regbyte & 0xC0) | (level as u8);
}
