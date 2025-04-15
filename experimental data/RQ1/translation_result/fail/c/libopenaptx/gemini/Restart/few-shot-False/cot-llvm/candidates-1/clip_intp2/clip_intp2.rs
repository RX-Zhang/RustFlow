#![allow(overflowing_literals)]

pub fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = ((2u32).wrapping_shl(p)).wrapping_sub(1);
    if (a.wrapping_add(1i32.wrapping_shl(p)) & !mask as i32) != 0 {
        (a >> 31) ^ (((1u32).wrapping_shl(p)).wrapping_sub(1) as i32)
    } else {
        a
    }
}
