#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1 << (shift.wrapping_sub(1) % 32);
    let mask: i32 = ((1 << ((shift.wrapping_add(1)) % 32)) as i64 - 1) as i32;
    let shifted = value.wrapping_add(rounding).wrapping_shr(shift % 32);
    let masked = value & mask;

    shifted.wrapping_sub(if masked == rounding { 1 } else { 0 })
}
