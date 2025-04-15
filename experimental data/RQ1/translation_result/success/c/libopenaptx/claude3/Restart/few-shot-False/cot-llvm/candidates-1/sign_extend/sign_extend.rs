#[inline]
pub fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 32u32.wrapping_sub(bits);
    let shl = val.wrapping_shl(shift);
    shl.wrapping_shr(shift as u32)
}
