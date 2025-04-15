#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1 << ((shift as i64).wrapping_sub(1) as u32 % 64);
    let mask: i64 = ((1 as i64) << ((shift as i64).wrapping_add(1) as u32 % 64)).wrapping_sub(1);
    let condition = (value & mask) == rounding;
    value.wrapping_add(rounding).wrapping_shr(shift % 64) - (if condition { 1 } else { 0 })
}
