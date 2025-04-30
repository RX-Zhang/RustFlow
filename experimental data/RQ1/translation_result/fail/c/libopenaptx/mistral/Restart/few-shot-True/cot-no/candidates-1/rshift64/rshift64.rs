#![allow(overflowing_literals)]

#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1i64.wrapping_shl(shift % 64);
    let mask: i64 = (1i64.wrapping_shl((shift % 64).wrapping_add(1))) - 1;
    let added = value.wrapping_add(rounding.wrapping_sub(1));
    let shifted = added.wrapping_shr(shift % 64);
    let masked = value & mask;
    let cmp = if masked == rounding.wrapping_sub(1) { 1 } else { 0 };
    shifted.wrapping_sub(cmp)
}
