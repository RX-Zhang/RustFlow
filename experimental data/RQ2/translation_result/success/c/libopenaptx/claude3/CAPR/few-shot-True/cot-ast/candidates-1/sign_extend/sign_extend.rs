#[inline]
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>() as u32).wrapping_sub(bits);
    let unsigned = (val as u32).wrapping_shl(shift);
    (unsigned as i32).wrapping_shr(shift)
}
