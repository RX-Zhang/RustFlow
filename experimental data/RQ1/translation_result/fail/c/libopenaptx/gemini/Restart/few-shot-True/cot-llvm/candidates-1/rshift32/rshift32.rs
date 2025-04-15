fn rshift32(value: i32, shift: i32) -> i32 {
    let rounding = (1 as i32) << shift;
    let mask = ((1 as i32) << (shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i32
}
