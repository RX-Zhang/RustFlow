fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 32u32.saturating_sub(bits % 32);
    (((val as u32).wrapping_shl(shift)) as i32).wrapping_shr(shift)
}
