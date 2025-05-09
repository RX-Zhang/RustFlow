
use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (Wrapping(a as u32) + Wrapping(1u32 << p)).0 & !((2u32 << p).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1i32 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift).wrapping_sub((((value & mask) == rounding) as i64))
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
