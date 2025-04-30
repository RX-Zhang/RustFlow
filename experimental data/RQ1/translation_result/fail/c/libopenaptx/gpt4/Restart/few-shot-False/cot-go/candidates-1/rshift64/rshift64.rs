fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl(shift - 1);
    let mask = (1i64.wrapping_shl(shift + 1)) - 1
    ((value.wrapping_add(rounding)).wrapping_shr(shift)) - ((value & mask) == rounding) as i64
}
