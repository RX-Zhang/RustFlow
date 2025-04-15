use std::num::Wrapping;

#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = Wrapping(1i64) << (shift.wrapping_sub(1) % 64) as usize;
    let mask = (Wrapping(1i64) << (shift.wrapping_add(1) % 64) as usize).0.wrapping_sub(1);
    let result = (Wrapping(value).0.wrapping_add(rounding.0) >> (shift % 64) as usize) as i64;
    let adjustment = ((value & mask) == rounding.0) as i64;
    result.wrapping_sub(adjustment)
}
