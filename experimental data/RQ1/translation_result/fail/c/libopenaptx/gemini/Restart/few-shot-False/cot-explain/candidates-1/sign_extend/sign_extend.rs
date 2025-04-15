
use std::mem::transmute;

#[inline]
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    let u: u32 = (val as u32).wrapping_shl(shift % 32);

    unsafe {
        let s: i32 = transmute::<u32, i32>(u)
        s.wrapping_shr(shift % 32)
    }

}
