#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl((shift.wrapping_sub(1)) as u32);
    let mask = (1i64.wrapping_shl((shift.wrapping_add(1)) as u32)).wrapping_sub(1);
    ((value.wrapping_add(rounding)).wrapping_shr(shift as u32)).wrapping_sub(if (value & mask) == rounding { 1 } else { 0 })
}
