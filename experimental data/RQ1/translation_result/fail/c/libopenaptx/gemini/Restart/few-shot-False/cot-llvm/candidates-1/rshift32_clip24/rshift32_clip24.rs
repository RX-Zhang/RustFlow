fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add((1u32).wrapping_shl(p))) & (!((2u32).wrapping_shl(p).wrapping_sub(1))) != 0 {
        (a >> 31) ^ ((1 << p) - 1) as i32
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = (1i32).wrapping_shl(shift.wrapping_sub(1) % 32);
    let mask: i32 = ((1i32).wrapping_shl(shift.wrapping_add(1) % 32)).wrapping_sub(1);
    let shifted = (value.wrapping_add(rounding)).wrapping_shr(shift % 32);

    let and_val = value & mask;
    if and_val == rounding {
        shifted.wrapping_sub(1)
    } else {
        shifted
    }
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
