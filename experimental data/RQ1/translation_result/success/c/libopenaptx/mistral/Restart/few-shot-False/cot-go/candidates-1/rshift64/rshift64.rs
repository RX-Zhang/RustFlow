fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1i64.wrapping_shl((shift.wrapping_sub(1)) % 64);
    let mask: i64 = (1i64.wrapping_shl((shift.wrapping_add(1)) % 64)).wrapping_sub(1);
    let result: i64 = (value.wrapping_add(rounding)).wrapping_shr(shift % 64);
    if (value & mask) == rounding {
        return result.wrapping_sub(1);
    }
    result
}
