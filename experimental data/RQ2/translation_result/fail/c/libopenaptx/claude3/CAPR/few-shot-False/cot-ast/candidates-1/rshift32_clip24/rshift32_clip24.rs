use std::num::Wrapping;
use std::ops::Shl;

#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (Wrapping(a as u32) + Wrapping(1u32.wrapping_shl(p))).0 & !((2u32.wrapping_shl(p)) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl(shift - 1);
    let mask = (1i32.wrapping_shl(shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i32
}

#[inline]
fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
