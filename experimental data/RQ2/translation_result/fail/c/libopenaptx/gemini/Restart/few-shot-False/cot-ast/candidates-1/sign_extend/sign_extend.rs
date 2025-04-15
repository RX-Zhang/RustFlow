use std::boxed::Box;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    let v = Box::new(((val as u32).wrapping_shl(shift)) as i32);
    v.wrapping_shr(shift)
}
