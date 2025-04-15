fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_wrapped = (a as u32).wrapping_add(1 << p);
    let mask = ((2 as u32) << p) - 1;
    if a_wrapped & !mask != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl(shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    let shifted = (value.wrapping_add(rounding) >> shift) as i32;
    shifted - if value & mask == rounding { 1 } else { 0 }
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
