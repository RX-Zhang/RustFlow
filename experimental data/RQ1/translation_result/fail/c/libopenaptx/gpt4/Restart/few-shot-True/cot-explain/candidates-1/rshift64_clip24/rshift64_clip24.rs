
use std::boxed::Box;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add((1 as u32).wrapping_shl(p)) & !(2_u32.wrapping_shl(p).wrapping_sub(1))) != 0 {
        (a >> 31) ^ ((1_i32.wrapping_shl(p)) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1_i64).wrapping_shl((shift as i64 - 1) as u32);
    let mask = (1_i64).wrapping_shl(shift.wrapping_add(1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift) - if (value & mask) == rounding { 1 } else { 0 }
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
