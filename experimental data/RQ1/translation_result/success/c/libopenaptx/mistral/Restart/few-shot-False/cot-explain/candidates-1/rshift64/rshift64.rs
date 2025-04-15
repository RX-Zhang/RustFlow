
#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1i64.wrapping_shl((shift.wrapping_sub(1)) as u32);
    let mask: i64 = (1i64.wrapping_shl((shift.wrapping_add(1)) as u32)).wrapping_sub(1);
    let shifted_value = value.wrapping_add(rounding).wrapping_shr(shift as u32);
    let mask_check = (value & mask) == rounding;

    if mask_check {
        shifted_value.wrapping_sub(1)
    } else {
        shifted_value
    }
}
