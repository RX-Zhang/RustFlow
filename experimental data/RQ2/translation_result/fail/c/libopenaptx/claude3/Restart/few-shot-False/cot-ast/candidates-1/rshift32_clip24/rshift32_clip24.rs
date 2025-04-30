use std::num::Wrapping;

#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_wrapped = Wrapping(a as u32);
    let one = Wrapping(1u32);
    let two = Wrapping(2u32);
    
    if (a_wrapped + (one << (p as usize))) & !((two << (p as usize)) - one) != Wrapping(0) {
        ((a >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a
    }
}

#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << shift.wrapping_sub(1);
    let mask = (1u32 << shift.wrapping_add(1)).wrapping_sub(1);
    ((value.wrapping_add(rounding as i32) >> shift) - ((value & (mask as i32)) == rounding as i32) as i32) as i32
}

#[inline]
fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
