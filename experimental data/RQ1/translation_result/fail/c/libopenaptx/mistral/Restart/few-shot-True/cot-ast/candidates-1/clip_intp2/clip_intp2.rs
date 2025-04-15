fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32: u32 = a as u32;
    let p_u32: u32 = p;

    let shifted = a_u32.wrapping_add(1 << p_u32);
    let mask = !( (2 << p_u32) -1 );

    if (shifted & mask) == 0 {
        a
    } else {
        ((a >> 31) as u32 ^ ((1 << p_u32) - 1)) as i32
    }
}
