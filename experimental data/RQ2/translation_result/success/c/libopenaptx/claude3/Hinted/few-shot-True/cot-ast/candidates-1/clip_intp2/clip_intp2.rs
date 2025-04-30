
#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32 = a as u32;
    let one_shifted = 1_u32.wrapping_shl(p);
    let two_shifted = 2_u32.wrapping_shl(p);
    if (a_u32.wrapping_add(one_shifted)) & !two_shifted.wrapping_sub(1) != 0 {
        (a >> 31) ^ ((1_i32 << p % 32) - 1)
    } else {
        a
    }
}
