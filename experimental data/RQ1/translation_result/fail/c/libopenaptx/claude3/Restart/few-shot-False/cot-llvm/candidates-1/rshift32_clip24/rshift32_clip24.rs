#![allow(arithmetic_overflow)]

pub fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32 = a as u32;
    let mask = ((2u32.wrapping_shl(p)).wrapping_sub(1)).wrapping_neg();
    if (a_u32.wrapping_add(1u32.wrapping_shl(p))) & mask != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p.try_into().unwrap())).wrapping_sub(1))
    } else {
        a
    }
}

pub fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl(shift.wrapping_sub(1))
    let mask = (1i32.wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift).wrapping_sub((value & mask == rounding) as i32)
}

pub fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
