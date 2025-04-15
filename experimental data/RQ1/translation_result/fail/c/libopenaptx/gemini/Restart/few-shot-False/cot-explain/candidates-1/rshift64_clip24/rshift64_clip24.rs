
use std::num::Wrapping;

#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((Wrapping(a as u32) + Wrapping(1u32 << p)) & Wrapping(!((2u32 << p) - 1))).0 != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1 << (shift - 1);
    let mask: i64 = ((1i64) << (shift + 1)) - 1;
    ((Wrapping(value) + Wrapping(rounding)).0 >> shift) - if (value & mask) == rounding { 1 } else { 0 }
}

#[inline]
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
