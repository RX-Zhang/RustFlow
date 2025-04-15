fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift: u32 = 32u32.wrapping_sub(bits);
    let shifted: i32 = val.wrapping_shl(shift % 32);
    shifted.wrapping_shr(shift % 32)
}
