fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 32u32.wrapping_sub(bits);
    let v_u = (val as u32).wrapping_shl(shift);
    let v_s = v_u as i32;
    v_s.wrapping_shr(shift)
}
