#![allow(overflowing_literals)]

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a as u32).wrapping_add(1u32 << p) & !((2u32 << p).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1i64 << (shift - 1);
    let mask: i64 = (1i64 << (shift + 1)).wrapping_sub(1);
    let mut result: i64 = (value.wrapping_add(rounding)) >> shift;
    if value & mask == rounding {
        result = result.wrapping_sub(1);
    }
    result
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
