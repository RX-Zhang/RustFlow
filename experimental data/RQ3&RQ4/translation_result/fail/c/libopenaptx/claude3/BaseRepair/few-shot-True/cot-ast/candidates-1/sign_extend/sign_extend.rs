#[inline]
fn sign_extend(val, bits: u32) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>() as u32).wrapping_sub(bits);
    let v_u = (val as u32).wrapping_shl(shift);
    (v_u as i32).wrapping_shr(shift)
}
