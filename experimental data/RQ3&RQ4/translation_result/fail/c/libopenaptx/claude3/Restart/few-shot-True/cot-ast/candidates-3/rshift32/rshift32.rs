#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl(shift.wrapping_sub(1) % 32);
    let mask = (1i32.wrapping_shl(shift.wrapping_add(1) % 32)).wrapping_sub(1);
    (value.wrapping_add(rounding).wrapping_shr(shift % 32))
        .wrapping_sub((value & mask == rounding) as i32)
}
