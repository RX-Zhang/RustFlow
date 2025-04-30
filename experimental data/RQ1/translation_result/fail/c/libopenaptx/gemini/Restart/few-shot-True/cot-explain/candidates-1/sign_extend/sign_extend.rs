
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (8 * core::mem::size_of::<i32>() as u32 - bits) as i32;
    let v = (val as u32).wrapping_shl(shift as u32) as i32;
    v.wrapping_shr(shift as u32) as i32
}

