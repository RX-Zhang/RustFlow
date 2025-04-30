
use std::num::Wrapping;

#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = (1_i32).wrapping_shl(shift.wrapping_sub(1) % 32);
    let mask: i32 = ((1_i32).wrapping_shl(shift.wrapping_add(1) % 32)).wrapping_sub(1);
    let shifted_value = Wrapping(value).0.wrapping_add(Wrapping(rounding).0).wrapping_shr(shift % 32);
    let mask_check = (value & mask) == rounding;

    if mask_check {
        shifted_value.wrapping_sub(1)
    } else {
        shifted_value
    }
}
