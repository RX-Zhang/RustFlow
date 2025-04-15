use std::mem::size_of;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (8 * size_of::<i32>() as u32).wrapping_sub(bits);
    let v = (val as u32).wrapping_shl(shift);
    (v as i32).wrapping_shr(shift)
}
