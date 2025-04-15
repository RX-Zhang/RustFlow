use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (Wrapping(a as u32) + Wrapping(1u32.wrapping_shl(p))).0 & !((2u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p as u32)).wrapping_sub(1))
    } else {
        a
    }
}
