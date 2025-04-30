fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1_i32.wrapping_shl(shift.wrapping_sub(1) % 32);
    let mask: i32 = (1_i32.wrapping_shl(shift.wrapping_add(1) % 32)).wrapping_sub(1);
    let shifted = value.wrapping_add(rounding).wrapping_shr(shift % 32);
    let condition = (value & mask) == rounding;

    shifted.wrapping_sub(if condition { 1 } else { 0 })
}
