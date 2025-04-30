
use std::boxed::Box;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + ((1 as u32) << p)) & !(((2 as u32) << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 as i64) << (shift - 1);
    let mask = ((1 as i64) << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
