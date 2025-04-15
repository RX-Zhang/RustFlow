fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 as i64) << (shift -1);
    let mask = ((1 as i64) << (shift + 1)) -1;
    ((value.wrapping_add(rounding)) >> shift as i64) - ((value & mask) == rounding) as i64
}
