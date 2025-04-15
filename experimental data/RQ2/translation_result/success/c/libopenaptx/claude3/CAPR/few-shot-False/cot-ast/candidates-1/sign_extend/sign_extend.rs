use std::num::Wrapping;

#[inline]
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>() as u32).wrapping_sub(bits);
    let v = Wrapping((val as u32).wrapping_shl(shift));
    (v.0 as i32).wrapping_shr(shift)
}
