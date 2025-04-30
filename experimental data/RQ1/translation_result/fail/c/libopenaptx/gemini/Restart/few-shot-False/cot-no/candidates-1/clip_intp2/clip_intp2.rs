#[inline]
pub fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32 = a as u32;
    let mask = (1u32.wrapping_shl(p).wrapping_mul(2)).wrapping_sub(1);
    
    if (a_u32.wrapping_add(1u32.wrapping_shl(p))) & !mask != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p as u32)) - 1)
    } else {
        a
    }
}
