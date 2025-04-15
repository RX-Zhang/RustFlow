use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let p = p % 32;
    let test = (Wrapping(a as u32) + Wrapping(1u32.wrapping_shl(p))).0 & !((2u32.wrapping_shl(p)).wrapping_sub(1));
    if test != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p)).wrapping_sub(1))
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let shift = shift % 32;
    let rounding = 1i32.wrapping_shl(shift.wrapping_sub(1));
    let mask = (1i32.wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1);
    let shifted = ((Wrapping(value) + Wrapping(rounding)).0 >> shift) as i32;
    let correction = (value & mask == rounding) as i32;
    shifted.wrapping_sub(correction)
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
