fn sign_extend(val: i32, bits: i32) -> i32 {
    let shift: u32 = (8 * std::mem::size_of::<i32>() as u32).wrapping_sub(bits as u32);
    let shifted_val = val.wrapping_shl(shift % 32);
    shifted_val.wrapping_shr(shift % 32)
}
