use std::mem;

#[inline]
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * (mem::size_of::<i32>() as u32) - bits;
    let v: i32 = (val as u32).wrapping_shl(shift % 32) as i32;
    v.wrapping_shr(shift % 32)
}
