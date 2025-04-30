use std::num::Wrapping;

#[inline]
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 32 - bits;
    let v = Wrapping((val as u32) << shift);
    (v.0 as i32) >> shift
}
