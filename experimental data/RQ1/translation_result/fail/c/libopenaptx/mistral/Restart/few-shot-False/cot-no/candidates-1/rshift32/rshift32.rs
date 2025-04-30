use std::num::Wrapping;

#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = Wrapping(1i32) << (shift.wrapping_sub(1) % 32) as usize;
    let mask = (Wrapping(1i32) << (shift.wrapping_add(1) % 32) as usize).0.wrapping_sub(1);
    let result = (Wrapping(value) + rounding).0.wrapping_shr(shift % 32);
    result.wrapping_sub(((value & mask) == rounding.0) as i32)
}
