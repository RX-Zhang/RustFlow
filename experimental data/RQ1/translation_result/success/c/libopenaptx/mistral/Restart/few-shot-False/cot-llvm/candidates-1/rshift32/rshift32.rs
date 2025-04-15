#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1_i32.wrapping_shl(shift.wrapping_sub(1) % 32);
    let mask = (1_i32.wrapping_shl(shift.wrapping_add(1) % 32)).wrapping_sub(1);
    let and_result = value & mask;
    let cmp = and_result == rounding;
    let added = value.wrapping_add(rounding);
    let shifted = added.wrapping_shr(shift % 32);
    shifted - (cmp as i32)
}
