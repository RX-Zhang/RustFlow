fn clip_intp2(a: i32, p: u32) -> i32 {
    let shift_left = 1 << p;
    if ((a as u32).wrapping_add(shift_left) & !((2 << p) - 1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    let value_add = value.wrapping_add(rounding)
    let shifted = value_add >> shift;
    if (value & mask) == rounding {
        shifted - 1
    } else {
        shifted
    }
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
