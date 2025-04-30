fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * 4 - bits;
    let v = (val as u32).wrapping_shl(shift % 32);
    (v as i32).wrapping_shr(shift % 32)
}
