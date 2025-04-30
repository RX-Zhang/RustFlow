#[inline]
pub fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a.wrapping_add(1_i32.wrapping_shl(p)) as u32) & !((2_u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1_i32.wrapping_shl(p)).wrapping_sub(1))
    } else {
        a
    }
}
