fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    (((val as u32).wrapping_shl(shift) as i32) as i64).wrapping_shr(shift as u32) as i32
}
