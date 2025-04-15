
fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32 = a as u32;
    let shift_val = 1u32.wrapping_shl(p);
    if (a_u32.wrapping_add(shift_val)) & !((2u32.wrapping_shl(p)) - 1) != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p)) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl((shift - 1) as u32);
    let mask = (1i64.wrapping_shl((shift + 1) as u32)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
