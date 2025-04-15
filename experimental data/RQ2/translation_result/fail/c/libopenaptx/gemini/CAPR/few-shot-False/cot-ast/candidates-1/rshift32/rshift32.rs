fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl(shift - 1);
    let mask = (1i32.wrapping_shl(shift + 1)) - 1;
    ((value.wrapping_add(rounding)).wrapping_shr(shift)) - ((value & mask) == rounding) as i32
}
