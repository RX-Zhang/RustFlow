fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1i64 << (shift.wrapping_sub(1) % 64) as i64;
    let mask: i64 = (1i64 << (shift.wrapping_add(1) % 64) as i64).wrapping_sub(1);
    ((value.wrapping_add(rounding)).wrapping_shr(shift % 64)) - if (value & mask) == rounding { 1 } else { 0 }
}
