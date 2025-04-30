use std::mem::transmute;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>() as u32) - bits;
    let v: u32 = unsafe { transmute(val) };
    let shifted = (v << shift) as i32;
    shifted >> shift
}
