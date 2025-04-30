use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (Wrapping(a as u32) + Wrapping(1u32 << p)) & !Wrapping((2u32 << p).wrapping_sub(1)) != Wrapping(0) {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift).wrapping_sub(if value & mask == rounding { 1 } else { 0 })
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
