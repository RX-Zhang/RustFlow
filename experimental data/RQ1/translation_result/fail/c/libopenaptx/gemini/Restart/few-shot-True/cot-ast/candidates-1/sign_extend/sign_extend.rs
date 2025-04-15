fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>() as u32 - bits) as i32;
    let mut v: i32;
    unsafe {
        let mut u: u32 = val as u32;
        u = u.wrapping_shl(shift as u32);
        v = u as i32;
        v = v.wrapping_shr(shift as u32);
    }
    v
}
