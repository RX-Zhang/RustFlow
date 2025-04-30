#![allow(overflowing_literals)]

#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (((a as u32).wrapping_add(1u32.wrapping_shl(p))) & (!((2u32.wrapping_shl(p)).wrapping_sub(1)))) != 0 {
        (a.wrapping_shr(31)) ^ ((1i32.wrapping_shl(p)) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1i64.wrapping_shl(shift.wrapping_sub(1) % 64);
    let mask: i64 = (1i64.wrapping_shl((shift).wrapping_add(1) % 64)).wrapping_sub(1);
    ((value.wrapping_add(rounding)).wrapping_shr(shift % 64)).wrapping_sub(if (value & mask) == rounding { 1 } else { 0 })
}

#[inline]
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
