fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask: u32 = (2u32.wrapping_shl(p)).wrapping_sub(1);
    if (a as u32).wrapping_add(1u32.wrapping_shl(p)) & !mask != 0 {
        (a.wrapping_shr(31)) ^ ((1i32.wrapping_shl(p)) - 1)
    } else {
        a
    }
}
