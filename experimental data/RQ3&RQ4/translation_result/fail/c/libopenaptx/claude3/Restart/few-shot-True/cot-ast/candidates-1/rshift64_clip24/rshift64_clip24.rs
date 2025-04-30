use std::num::Wrapping;

fn clip_intp2(a, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32.wrapping_shl(p))) & !((2u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1i32 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
