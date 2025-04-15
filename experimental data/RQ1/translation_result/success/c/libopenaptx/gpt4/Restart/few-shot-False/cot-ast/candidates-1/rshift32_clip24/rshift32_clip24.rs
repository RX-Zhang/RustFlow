use std::convert::TryInto;

#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (((a as u32).wrapping_add((1u32).wrapping_shl(p))) & (!((2u32).wrapping_shl(p)).wrapping_sub(1))) != 0 {
        (a.wrapping_shr(31)) ^ (((1i32).wrapping_shl(p)) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = (1i32).wrapping_shl(shift.wrapping_sub(1)) as i32;
    let mask: i32 = ((1i32).wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1) as i32;
    let condition = (value & mask) == rounding;
    (value.wrapping_add(rounding).wrapping_shr(shift)) - (if condition { 1 } else { 0 })
}

#[inline]
fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
