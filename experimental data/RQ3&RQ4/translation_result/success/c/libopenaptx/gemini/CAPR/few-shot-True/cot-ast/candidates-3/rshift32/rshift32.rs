fn rshift32(value: i32, shift: u32) -> i32 {
    let s = shift & 31;
    let rounding = 1i32.wrapping_shl(s.wrapping_sub(1));
    let mask = 1i32.wrapping_shl(s.wrapping_add(1)).wrapping_sub(1);
    (value.wrapping_add(rounding) >> s) - ((value & mask) == rounding) as i32
}
