
use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = ((1u64 << (p + 1)) - 1) as u32;
    if (Wrapping(a as u32) + Wrapping(1u32 << p)).0 & !mask != 0 {
        (a >> 31) ^ ((1i32 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    let result = ((Wrapping(value) + Wrapping(rounding)).0 >> shift) as i32;
    result - ((value & mask) == rounding) as i32
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
