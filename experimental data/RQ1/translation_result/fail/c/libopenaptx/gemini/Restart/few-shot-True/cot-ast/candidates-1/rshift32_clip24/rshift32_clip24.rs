use std::convert::TryInto;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let p_val: u32 = p;
    if ((a as u32).wrapping_add(1u32 << p_val)) & !((2u32 << p_val) - 1) != 0 {
        (a >> 31) ^ ((1 << p_val) - 1) as i32
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = (1 as i32) << (shift -1) as i32;
    let mask = ((1 as i32) << (shift + 1) as i32) - 1;
    ((value + rounding) >> shift as i32) - ((value & mask) == rounding) as i32
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
