#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl(shift.wrapping_sub(1) % 64);
    let mask = (1i64.wrapping_shl(shift.wrapping_add(1) % 64)).wrapping_sub(1);
    (value.wrapping_add(rounding) >> (shift % 64)).wrapping_sub((value & mask == rounding) as i64)
}
