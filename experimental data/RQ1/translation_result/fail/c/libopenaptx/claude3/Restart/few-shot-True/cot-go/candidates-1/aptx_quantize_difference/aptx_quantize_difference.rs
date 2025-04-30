fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    (value >> shift).clamp(-8388608, 8388607)
}

pub fn process_dither(dither: i32) -> i32 {
    let d = rshift32_clip24(((dither as i64).wrapping_mul(dither as i64) >> 32) as i32, 7).wrapping_sub(1 << 23);
    d
}
